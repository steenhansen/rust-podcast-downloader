#![warn(unused_extern_crates)]

#[allow(unused)]
use log::{debug, info, trace, warn};

mod app_ui;

pub mod chunks;
pub mod components;
pub mod consts;
pub mod dialogs;
pub mod episodes;
pub mod events;
pub mod files;
pub mod globals;
pub mod misc;
pub mod podcasts;
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
        misc::tui_term::init(consts::const_globals::DEBUG_FILE).expect("tui-init-err");

    files::file_log::reqwest_trace_off(LevelFilter::Info); // this stops all Debug & Trace logging from ReQwest

    let tick_rate = Duration::from_millis(100);
    let mut app = state::app_state::DownApp::default();

    app.hover_element = state::app_state::HOVER_NONE.to_string();
    let res = run_app(&mut the_terminal, app, tick_rate);
    misc::tui_term::restore(the_terminal, res).expect("tui-restore-err");
    Ok(())
}

fn run_app<B: Backend>(
    the_terminal: &mut Terminal<B>,
    mut the_app: state::app_state::DownApp,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    podcasts::podcast_state::get_dirs_of_podcasts(&mut the_app);

    loop {
        the_terminal
            .draw(|f| app_ui::draw_ui(f, &mut the_app))
            .expect("tui-draw-err");

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout).expect("tui-poll-err") {
            let mut the_frame = the_terminal.get_frame();
            let finish = events::ev_all::all_events_done(&mut the_frame, &mut the_app);
            if finish {
                return Ok(());
            }
        }
        state::state_reify::change_app_state_type(&mut the_app);
        chunks::episode_threads::check_start_down(&mut the_app);
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
