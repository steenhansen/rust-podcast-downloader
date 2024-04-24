#![warn(unused_extern_crates)]

#[allow(unused)]
use log::{debug, info, trace, warn};

mod app_show;
mod app_state;
mod app_ui;
mod area_rects;
mod areas_consts;
mod chunk_range;
mod close_error;
mod const_globals;
mod episode_colors;
mod episode_scroll;
mod episode_threads;
mod episodes_chunks;
mod ev_after_draw;
mod ev_all;
mod ev_click;
mod ev_key;
mod ev_scroll;
mod file_log;
mod g_current_active;
mod g_pause_io;
mod g_resource_speed;
mod misc_fun;
mod podcast_episodes;
mod podcast_files;
mod podcast_scroll;
mod render_controls;
mod render_inputs;
mod render_misc;
mod rss_xml;
mod the_types;
mod tui_term;

use log::LevelFilter;

use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use ratatui::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = tui_term::init(const_globals::DEBUG_FILE).expect("tui-init-err");

    file_log::reqwest_trace_off(LevelFilter::Info); // this stops all Debug & Trace logging from ReQwest

    let tick_rate = Duration::from_millis(100);
    let app = app_state::DownApp::default();
    let res = run_app(&mut terminal, app, tick_rate);
    tui_term::restore(terminal, res).expect("tui-restore-err");
    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut the_app: app_state::DownApp,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    podcast_scroll::get_dirs_of_podcasts(&mut the_app);

    loop {
        terminal
            .draw(|f| app_ui::draw_ui(f, &mut the_app))
            .expect("tui-draw-err");

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout).expect("tui-poll-err") {
            let mut the_frame = terminal.get_frame();
            let finish = ev_all::all_events_done(&mut the_frame, &mut the_app);
            if finish {
                return Ok(());
            }
        }
        ev_after_draw::after_ui(&mut the_app);
        episode_threads::check_start_down(&mut the_app);
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
