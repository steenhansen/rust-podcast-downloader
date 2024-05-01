#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::components::btn_every;
use crate::components::btn_new;
use crate::components::btn_quit;
use crate::components::btn_stop;
use crate::components::chars_feed;
use crate::components::chars_status;
use crate::components::chars_title;
use crate::components::checkbox_pause;
use crate::components::input_address;
use crate::components::input_name;
use crate::state::app_state;

use crate::episodes::episode_state;
use crate::podcasts::podcast_state;

use crate::components::radio_resource;
use crate::consts::the_types;
use crate::dialogs::ok_dialogs;
use crate::globals::g_pause_io;

use ratatui::prelude::*;

fn dim_background(the_app: &app_state::DownApp) -> bool {
    let ui_state = the_app.ui_state;
    if ui_state == the_types::UiState::State101ReadingRss
        || ui_state == the_types::UiState::State102ShowWaiting
        || ui_state == the_types::UiState::State201EveryEpisode
        || ui_state == the_types::UiState::State202SureEveryEpisode
        || ui_state == the_types::UiState::State301WaitForPopErrorClose
        || ui_state == the_types::UiState::State501Help
    {
        return true;
    }
    false
}

pub fn draw_ui(console_frame: &mut Frame, the_app: &mut app_state::DownApp) {
    let dim_background = dim_background(the_app);
    let is_downloading_paused = g_pause_io::is_cur_paused();

    chars_title::show_title(
        console_frame,
        the_app,
        dim_background,
        is_downloading_paused,
    );
    input_address::show_address(
        console_frame,
        the_app,
        dim_background,
        is_downloading_paused,
    );

    input_name::show_name(
        console_frame,
        the_app,
        dim_background,
        is_downloading_paused,
    );

    btn_new::show_new(
        console_frame,
        the_app,
        dim_background,
        is_downloading_paused,
    );

    btn_every::show_every(console_frame, the_app);

    btn_stop::show_stop(console_frame, the_app);

    radio_resource::show_resources(
        console_frame,
        the_app,
        dim_background,
        is_downloading_paused,
    );

    podcast_state::state_of_podcasts(
        console_frame,
        the_app,
        dim_background,
        is_downloading_paused,
    );
    episode_state::state_of_episodes(
        console_frame,
        the_app,
        dim_background,
        is_downloading_paused,
    );
    chars_status::show_status(
        console_frame,
        the_app,
        dim_background,
        is_downloading_paused,
    );

    chars_feed::show_feed(
        console_frame,
        the_app,
        dim_background,
        is_downloading_paused,
    );

    checkbox_pause::show_pause(console_frame, the_app);

    ok_dialogs::render_help(console_frame, the_app);

    ok_dialogs::render_error_close(console_frame, the_app);
    ok_dialogs::render_disk_space(console_frame, the_app);
    btn_quit::show_quit(console_frame, the_app);
}
