#![warn(unused_extern_crates)]

#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_globals;

pub mod app;
pub mod components;
pub mod consts;
pub mod dialogs;
pub mod events;
pub mod files;
pub mod globals;
pub mod media;
pub mod misc;
pub mod state;

use log::LevelFilter;
use ratatui::prelude::*;

use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
fn main() -> Result<(), Box<dyn Error>> {
    let mut the_terminal =
        misc::misc_tui::tui_init(consts::const_globals::DEBUG_FILE).expect("tui-init-err");

    files::file_log::log_trace_off(LevelFilter::Info); // this stops all Debug & Trace logging from ReQwest

    let tick_rate = Duration::from_millis(100);
    let mut app = state::state_app::DownApp::default();

    app.hover_element = state::state_app::HOVER_NONE.to_string();
    let res = run_app(&mut the_terminal, app, tick_rate);
    misc::misc_tui::tui_restore(the_terminal, res).expect("tui-tui_restore-err");
    Ok(())
}

fn run_app<B: Backend>(
    the_terminal: &mut Terminal<B>,
    mut the_app: state::state_app::DownApp,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    components::podcasts::podcast_media::media_sorted_podcasts(&mut the_app);
    the_app.podcast_file_types =
        components::podcasts::podcast_media::media_file_types(const_globals::ROOT_DIR);

    loop {
        match the_terminal.draw(|f| app::draw_ui(f, &mut the_app)) {
            Ok(_complete_frame) => {}
            Err(e) => warn!(" aaaa1111 {:?}", e),
        }

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        match crossterm::event::poll(timeout) {
            Ok(is_a_timeout) => {
                if is_a_timeout {
                    let mut the_frame = the_terminal.get_frame();
                    let finish = events::ev_all::all_events(&mut the_frame, &mut the_app);
                    if finish {
                        return Ok(());
                    }
                }
            }
            Err(e) => warn!(" bbb22 {:?}", e),
        }

        state::state_reify::reify_type(&mut the_app);
        media::media_threads::threads_limit(&mut the_app);
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
