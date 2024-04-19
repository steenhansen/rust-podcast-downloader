#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports)]

mod app_ui;
mod area_rects;
mod areas_consts;
mod close_error;
mod const_globals;
mod episode_scroll;
mod episode_threads;
mod episodes_files;
mod ev_after_draw;
mod ev_all;
mod ev_click;
mod ev_key;
mod ev_scroll;
mod file_log;
mod g_current_active;
mod g_resource_speed;
mod misc_fun;
mod podcast_files;
mod podcast_scroll;
mod render_add;
mod render_misc;
mod render_radio_check;
mod rss_xml;
mod the_types;
mod tui_term;

mod type_partial_range_iter;

#[allow(unused)]
use log::{debug, info, trace, warn};

use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use ratatui::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = tui_term::init(const_globals::DEBUG_FILE)?;

    let tick_rate = Duration::from_millis(100);
    let app = app_ui::DownApp::default();
    let res = run_app(&mut terminal, app, tick_rate);
    tui_term::restore(terminal, res)?;
    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut the_app: app_ui::DownApp,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    podcast_scroll::get_dirs_of_podcasts(&mut the_app);

    loop {
        terminal.draw(|f| app_ui::draw_ui(f, &mut the_app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            let mut the_frame = terminal.get_frame();
            let finish = ev_all::all_events_done(&mut the_frame, &mut the_app);
            if finish {
                return Ok(());
            }
        }
        ev_after_draw::after_ui(&mut the_app);

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
