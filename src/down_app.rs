#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports)]
use crate::close_error;
use crate::const_areas;
use crate::episode_scroll;
use crate::input_box;
use crate::podcast_scroll;
use crate::the_types;
#[allow(unused)]
use log::{debug, info, trace, warn};
use ratatui::{prelude::*, widgets::*};
use std::collections::HashMap;
#[derive(Default)]
pub struct DownApp {
    pub podcast_name: String,
    pub podcast_url: String,
    pub ui_state: the_types::UiState,
    pub local_episode_files: HashMap<String, String>, //the_app.local_episode_files {"rss_url.txt": "rss_url.txt"}
    pub files_2_download: HashMap<String, String>,    //  the_app.files_2_download {}
    pub episode_2_url: HashMap<String, String>,
    pub selected_episode: String,
    pub selected_podcast: String,
    pub scrolled_podcasts: usize,
    pub scrolled_episodes: usize,
    pub ordered_podcasts: Vec<String>,
    pub ordered_episodes: Vec<String>,
    pub state_scroll_podcasts: ScrollbarState,
    pub state_scroll_episodes: ScrollbarState,
    pub cur_error: String,
}

#[allow(clippy::too_many_lines, clippy::cast_possible_truncation)]
pub fn draw_ui(console_frame: &mut Frame, the_app: &mut DownApp) {
    input_box::render_url(
        console_frame,
        const_areas::NEW_PODCAST_URL,
        the_app,
        "New Podcast URL───https://www.nasa.gov/feeds/iotd-feed",
    );
    input_box::render_name(
        console_frame,
        const_areas::NEW_PODCAST_NAME,
        the_app,
        "New Podcast Name───Nasa-Images",
    );
    input_box::render_button(
        console_frame,
        const_areas::ADD_AREA,
        the_app,
        " Add Podcast",
    );

    // let size = console_frame.size();

    // let chunks = Layout::vertical([
    //     Constraint::Min(1),
    //     Constraint::Percentage(25),
    //     Constraint::Percentage(25),
    //     Constraint::Percentage(25),
    //     Constraint::Percentage(25),
    // ])
    // .split(size);

    input_box::render_title(console_frame, const_areas::TITLE_AREA, the_app, "The Title");

    podcast_scroll::render_pod_list(
        console_frame,
        const_areas::PODCAST_AREA,
        the_app,
        String::from("Choose Podcast to Download"),
    );
    podcast_scroll::podcasts_vscroll(console_frame, const_areas::PODCAST_AREA, the_app);

    if the_app.selected_podcast != "" {
        if the_app.ui_state == the_types::UiState::State103ShowEpisodes {
            episode_scroll::render_epi_list(
                console_frame,
                const_areas::EPISODE_AREA,
                the_app,
                String::from("Select Episodes to Download"),
            );
            episode_scroll::episode_vscroll(console_frame, const_areas::EPISODE_AREA, the_app);
        } else {
            episode_scroll::render_epi_list_empty(
                console_frame,
                const_areas::EPISODE_AREA,
                the_app,
                String::from(""),
            );
        }
    }

    let up_right_area = close_error::get_end_prog_area(console_frame);
    input_box::render_close(console_frame, up_right_area, "X");

    close_error::render_pop_up_close(the_app, console_frame);
}
