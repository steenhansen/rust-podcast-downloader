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

use crate::render_radio_check;
use crate::the_types;
#[allow(unused)]
use log::{debug, info, trace, warn};
use ratatui::{prelude::*, widgets::*};
use std::collections::HashMap;
#[derive(Default, Debug)]
pub struct DownApp {
    pub podcast_name: String,
    pub podcast_url: String,
    pub ui_state: the_types::UiState,
    pub local_episode_files: HashMap<String, String>, //the_app.local_episode_files {"podcast_url.rss": "podcast_url.rss"}
    pub files_downloading: u16,
    pub episode_2_url: HashMap<String, String>,
    pub selected_episode: String,
    pub selected_podcast: String,
    pub scrolled_podcasts: usize,
    pub scrolled_episodes: usize,
    pub ordered_podcasts: Vec<String>,
    pub ordered_episodes: Vec<String>, // "0024-a_pod", "0023-crime"
    pub state_scroll_podcasts: ScrollbarState,
    pub state_scroll_episodes: ScrollbarState,
    pub cur_error: String,
    pub fast_med_slow: u16,
    pub init_erased_dirs: HashMap<String, bool>,
    pub int_prefix: bool,
}

#[allow(clippy::too_many_lines, clippy::cast_possible_truncation)]
pub fn draw_ui(console_frame: &mut Frame, the_app: &mut DownApp) {
    show_title(console_frame, the_app);
    show_add(console_frame, the_app);
    show_resources(console_frame, the_app);
    show_quit(console_frame);
    show_podcasts(console_frame, the_app);
    show_episodes(console_frame, the_app);
    show_status(console_frame, the_app);
    show_prefix(console_frame, the_app);

    close_error::render_pop_up_close(console_frame, the_app);
}

fn show_title(console_frame: &mut Frame, the_app: &mut DownApp) {
    render_misc::render_title(
        console_frame,
        areas_consts::TITLE_AREA,
        the_app,
        "The Title",
    );
}

fn show_add(console_frame: &mut Frame, the_app: &mut DownApp) {
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
        " Add Podcast",
    );
}

fn show_podcasts(console_frame: &mut Frame, the_app: &mut DownApp) {
    let elastic_pod_area = area_rects::get_podcast_area(console_frame);
    podcast_scroll::render_pod_list(
        console_frame,
        elastic_pod_area,
        the_app,
        String::from("Choose Podcast to Download"),
    );
    podcast_scroll::podcasts_vscroll(console_frame, elastic_pod_area, the_app);
}

fn show_episodes(console_frame: &mut Frame, the_app: &mut DownApp) {
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

fn show_resources(console_frame: &mut Frame, the_app: &mut DownApp) {
    render_radio_check::render_resources(
        console_frame,
        areas_consts::RADIO_AREA,
        the_app,
        "Resources",
    );
}

fn show_quit(console_frame: &mut Frame) {
    let up_right_area = area_rects::get_quit_area(console_frame);
    render_misc::render_close(console_frame, up_right_area, "X");
}

fn show_status(console_frame: &mut Frame, the_app: &mut DownApp) {
    let status_area = area_rects::get_status_area(console_frame);
    let num_downloading = g_current_active::get_gss();
    let stat_mess = format!(" Active Downloading Files {:?}", num_downloading);
    render_misc::render_status(console_frame, status_area, the_app, &stat_mess);
    // red if more than 1
}

fn show_prefix(console_frame: &mut Frame, the_app: &mut DownApp) {
    if the_app.selected_podcast != "" {
        let is_in_prefix =
            podcast_scroll::is_int_prefix(const_globals::ROOT_DIR, &the_app.selected_podcast);
        let prefix_area = area_rects::get_prefix_area(console_frame);
        if is_in_prefix {
            render_radio_check::render_prefix(
                console_frame,
                prefix_area,
                the_app,
                "[X] Use Integer Prefix",
            );
        } else {
            render_radio_check::render_prefix(
                console_frame,
                prefix_area,
                the_app,
                "[O] Use Integer Prefix",
            );
        }
    }
}
