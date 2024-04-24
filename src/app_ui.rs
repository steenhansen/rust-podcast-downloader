#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::app_show;
use crate::app_state;
use crate::close_error;
use crate::g_pause_io;
use crate::the_types;

use ratatui::prelude::*;

fn dim_background(the_app: &app_state::DownApp) -> bool {
    let ui_state = the_app.ui_state;
    if ui_state == the_types::UiState::State101ReadingRss
        || ui_state == the_types::UiState::State102ShowWaiting
        || ui_state == the_types::UiState::State201AllEpisodes
        || ui_state == the_types::UiState::State202SureAllEpisodes
        || ui_state == the_types::UiState::State301WaitForPopErrorClose
    {
        return true;
    }
    false
}

#[allow(clippy::too_many_lines, clippy::cast_possible_truncation)]
pub fn draw_ui(console_frame: &mut Frame, the_app: &mut app_state::DownApp) {
    let dim_background = dim_background(the_app);
    let is_downloading_paused = g_pause_io::is_cur_paused();

    app_show::show_title(
        console_frame,
        the_app,
        dim_background,
        is_downloading_paused,
    );

    app_show::show_add(
        console_frame,
        the_app,
        dim_background,
        is_downloading_paused,
    );
    app_show::show_resources(
        console_frame,
        the_app,
        dim_background,
        is_downloading_paused,
    );

    app_show::show_podcasts(
        console_frame,
        the_app,
        dim_background,
        is_downloading_paused,
    );
    app_show::show_episodes(
        console_frame,
        the_app,
        dim_background,
        is_downloading_paused,
    );
    app_show::show_status(
        console_frame,
        the_app,
        dim_background,
        is_downloading_paused,
    );
    app_show::show_pause(console_frame, the_app, dim_background);

    close_error::render_error_close(console_frame, the_app);
    close_error::render_all_ok(console_frame, the_app);
    app_show::show_quit(console_frame);
}
