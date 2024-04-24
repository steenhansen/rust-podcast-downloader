#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::app_state;
use crate::area_rects;
use crate::areas_consts;
use crate::const_globals;
use crate::episode_scroll;
use crate::g_current_active;
use crate::g_pause_io;
use crate::podcast_scroll;
use crate::render_controls;
use crate::render_inputs;
use crate::render_misc;
use crate::the_types;

use ratatui::prelude::*;
use std::time::SystemTime;
//use time::OffsetDateTime;

pub fn show_title(
    console_frame: &mut Frame,
    the_app: &mut app_state::DownApp,
    dim_background: bool,
    is_downloading_paused: bool,
) {
    let the_title = "Terminal Rust Podcast Downloader";
    let title_area = area_rects::get_title_area(console_frame, the_title);
    let mut wait_color = const_globals::TITLE_COLOR;
    if is_downloading_paused {
        wait_color = const_globals::PAUSE_COLOR;
    } else if dim_background {
        wait_color = const_globals::DIMMED_BACKGROUND_WAIT;
    }
    render_misc::render_title(
        console_frame,
        title_area,
        the_app,
        "Terminal Rust Podcast Downloader",
        wait_color,
    );
}

pub fn show_quit(console_frame: &mut Frame) {
    let up_right_area = area_rects::get_quit_area(console_frame);

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system-time-err");
    let elapsed_sed = now.as_secs();
    let show_alive = elapsed_sed % 5;
    let color_spinner = match show_alive {
        0 => Color::Green,
        1 => Color::LightRed,
        2 => Color::Yellow,
        3 => Color::LightMagenta,
        _ => Color::Cyan,
    };
    render_misc::render_quit(console_frame, up_right_area, " X ", color_spinner);
}

pub fn show_pause(
    console_frame: &mut Frame,
    the_app: &mut app_state::DownApp,
    dim_background: bool,
) {
    if the_app.selected_podcast != "" {
        let is_currently_paused = g_pause_io::is_cur_paused();
        let pause_area = areas_consts::PAUSE_AREA;
        if is_currently_paused {
            render_controls::render_pause(
                console_frame,
                pause_area,
                the_app,
                "[✓] Downloads Paused",
                dim_background,
            );
        } else {
            render_controls::render_pause(
                console_frame,
                pause_area,
                the_app,
                "[ ] Pause Downloads",
                dim_background,
            );
        }
    }
}

pub fn show_podcasts(
    console_frame: &mut Frame,
    the_app: &mut app_state::DownApp,
    dim_background: bool,
    is_downloading_paused: bool,
) {
    let elastic_pod_area = area_rects::get_podcast_area(console_frame);
    let mut wait_color = const_globals::NORMAL_BORDER_COL;
    if is_downloading_paused {
        wait_color = const_globals::PAUSE_COLOR;
    } else if dim_background {
        wait_color = const_globals::DIMMED_BACKGROUND_WAIT;
    }
    podcast_scroll::render_pod_list(
        console_frame,
        elastic_pod_area,
        the_app,
        String::from("Choose Podcast to Download"),
        wait_color,
    );
    podcast_scroll::podcasts_vscroll(console_frame, elastic_pod_area, the_app);
}

pub fn show_episodes(
    console_frame: &mut Frame,
    the_app: &mut app_state::DownApp,
    dim_background: bool,
    is_downloading_paused: bool,
) {
    if the_app.selected_podcast != "" {
        let elastic_epi_area = area_rects::get_episode_area(console_frame);
        let mut wait_color = const_globals::NORMAL_BORDER_COL;
        if is_downloading_paused {
            wait_color = const_globals::PAUSE_COLOR;
        } else if dim_background {
            wait_color = const_globals::DIMMED_BACKGROUND_WAIT;
        }
        if the_app.ui_state == the_types::UiState::State103ShowEpisodes {
            episode_scroll::render_epi_list(
                console_frame,
                elastic_epi_area,
                the_app,
                String::from("Select Episodes to Download"),
                wait_color,
            );
            episode_scroll::episode_vscroll(console_frame, elastic_epi_area, the_app);
        } else {
            episode_scroll::render_epi_list_empty(
                console_frame,
                elastic_epi_area,
                the_app,
                String::from(""),
                wait_color,
            );
        }
    }
}

pub fn show_status(
    console_frame: &mut Frame,
    the_app: &mut app_state::DownApp,
    dim_background: bool,
    is_downloading_paused: bool,
) {
    let status_area = area_rects::get_status_area(console_frame);
    let mut wait_color = const_globals::NORMAL_BORDER_COL;
    if is_downloading_paused {
        wait_color = const_globals::PAUSE_COLOR;
    } else if dim_background {
        wait_color = const_globals::DIMMED_BACKGROUND_WAIT;
    }
    let num_downloading = g_current_active::active_downloading();
    let num_waiting = the_app.download_deque.len();
    let stat_mess = format!(
        " Active Downloading Files {:?}, Waiting {:?}",
        num_downloading, num_waiting
    );
    render_misc::render_status(console_frame, status_area, the_app, &stat_mess, wait_color);
}

pub fn show_resources(
    console_frame: &mut Frame,
    the_app: &mut app_state::DownApp,
    dim_background: bool,
    is_downloading_paused: bool,
) {
    let mut wait_color = const_globals::NORMAL_BORDER_COL;
    if is_downloading_paused {
        wait_color = const_globals::PAUSE_COLOR;
    } else if dim_background {
        wait_color = const_globals::DIMMED_BACKGROUND_WAIT;
    }
    render_controls::render_resources(
        console_frame,
        areas_consts::RADIO_AREA,
        the_app,
        "Internet Resource Load",
        wait_color,
    );
}

pub fn show_add(
    console_frame: &mut Frame,
    the_app: &mut app_state::DownApp,
    dim_background: bool,
    is_downloading_paused: bool,
) {
    let mut wait_color = const_globals::NORMAL_BORDER_COL;
    if is_downloading_paused {
        wait_color = const_globals::PAUSE_COLOR;
    } else if dim_background {
        wait_color = const_globals::DIMMED_BACKGROUND_WAIT;
    }
    render_inputs::render_url(
        console_frame,
        areas_consts::NEW_URL_AREA,
        the_app,
        "New Podcast URL───https://www.nasa.gov/feeds/iotd-feed",
        wait_color,
    );
    render_inputs::render_name(
        console_frame,
        areas_consts::NEW_NAME_AREA,
        the_app,
        "New Podcast Name───Nasa-Images",
        wait_color,
    );
    render_inputs::render_add_podcast(
        console_frame,
        areas_consts::ADD_PODCAST_AREA,
        the_app,
        "\n Add Podcast",
        wait_color,
    );

    render_inputs::render_all_podcast(
        console_frame,
        areas_consts::ALL_PODCAST_AREA,
        the_app,
        "\n Download All Episodes",
        wait_color,
    );
}
