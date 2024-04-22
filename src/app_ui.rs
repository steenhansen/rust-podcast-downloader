#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports)]
use crate::area_rects;
use crate::areas_consts;
use crate::close_error;
use crate::const_globals;
use crate::episode_scroll;
use crate::g_current_active;
use crate::podcast_scroll;
use crate::render_add;
use crate::render_misc;

use crate::app_state;
use crate::render_speed;
use crate::the_types;
#[allow(unused)]
use log::{debug, info, trace, warn};
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
    show_title(console_frame, the_app);
    show_add(console_frame, the_app, dim_background);
    show_resources(console_frame, the_app, dim_background);
    show_quit(console_frame);
    show_podcasts(console_frame, the_app, dim_background);
    show_episodes(console_frame, the_app);
    show_status(console_frame, the_app);
    show_prefix(console_frame, the_app, dim_background);

    close_error::render_error_close(console_frame, the_app);
    close_error::render_all_ok(console_frame, the_app);
}

fn show_title(console_frame: &mut Frame, the_app: &mut app_state::DownApp) {
    render_misc::render_title(
        console_frame,
        areas_consts::TITLE_AREA,
        the_app,
        "The Title",
    );
}

fn show_add(console_frame: &mut Frame, the_app: &mut app_state::DownApp, dim_background: bool) {
    render_add::render_url(
        console_frame,
        areas_consts::NEW_URL_AREA,
        the_app,
        "New Podcast URL───https://www.nasa.gov/feeds/iotd-feed",
    );
    render_add::render_name(
        console_frame,
        areas_consts::NEW_NAME_AREA,
        the_app,
        "New Podcast Name───Nasa-Images",
    );
    render_add::render_add_podcast(
        console_frame,
        areas_consts::ADD_PODCAST_AREA,
        the_app,
        "\n Add Podcast",
        dim_background,
    );

    render_add::render_all_podcast(
        console_frame,
        areas_consts::ALL_PODCAST_AREA,
        the_app,
        "\n Download All Episodes",
        dim_background,
    );
}

fn show_resources(
    console_frame: &mut Frame,
    the_app: &mut app_state::DownApp,
    dim_background: bool,
) {
    render_speed::render_resources(
        console_frame,
        areas_consts::RADIO_AREA,
        the_app,
        "Internet Resource Load",
        dim_background,
    );
}
fn show_prefix(console_frame: &mut Frame, the_app: &mut app_state::DownApp, dim_background: bool) {
    if the_app.selected_podcast != "" {
        let is_in_prefix =
            podcast_scroll::is_int_prefix(const_globals::ROOT_DIR, &the_app.selected_podcast);
        let prefix_area = areas_consts::PREFIX_AREA;
        if is_in_prefix {
            render_speed::render_prefix(
                console_frame,
                prefix_area,
                the_app,
                "[✓] Names Integer Prefixed",
                dim_background,
            );
        } else {
            render_speed::render_prefix(
                console_frame,
                prefix_area,
                the_app,
                "[ ] Names Integer Prefixed",
                dim_background,
            );
        }
    }
}

fn show_podcasts(
    console_frame: &mut Frame,
    the_app: &mut app_state::DownApp,
    dim_background: bool,
) {
    let elastic_pod_area = area_rects::get_podcast_area(console_frame);
    podcast_scroll::render_pod_list(
        console_frame,
        elastic_pod_area,
        the_app,
        String::from("Choose Podcast to Download"),
        dim_background,
    );
    podcast_scroll::podcasts_vscroll(console_frame, elastic_pod_area, the_app);
}

fn show_episodes(console_frame: &mut Frame, the_app: &mut app_state::DownApp) {
    if the_app.selected_podcast != "" {
        let elastic_epi_area = area_rects::get_episode_area(console_frame);
        if the_app.ui_state == the_types::UiState::State103ShowEpisodes {
            episode_scroll::render_epi_list(
                console_frame,
                elastic_epi_area,
                the_app,
                String::from("Select Episodes to Download"),
            );
            episode_scroll::episode_vscroll(console_frame, elastic_epi_area, the_app);
        } else {
            episode_scroll::render_epi_list_empty(
                console_frame,
                elastic_epi_area,
                the_app,
                String::from(""),
            );
        }
    }
}

fn show_quit(console_frame: &mut Frame) {
    let up_right_area = area_rects::get_quit_area(console_frame);
    render_misc::render_close(console_frame, up_right_area, "X");
}

fn show_status(console_frame: &mut Frame, the_app: &mut app_state::DownApp) {
    let status_area = area_rects::get_status_area(console_frame);
    let num_downloading = g_current_active::active_downloading();
    let num_waiting = the_app.download_deque.len();
    let stat_mess = format!(
        " Active Downloading Files {:?}, Waiting {:?}",
        num_downloading, num_waiting
    );
    render_misc::render_status(console_frame, status_area, the_app, &stat_mess);
}
