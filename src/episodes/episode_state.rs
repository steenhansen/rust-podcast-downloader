#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::episodes::episode_colors;
use crate::episodes::episode_event;
use crate::episodes::episode_show;

use crate::consts::area_rects;
use crate::consts::const_globals;
use crate::consts::the_types;
use crate::globals::g_current_active;
use crate::state::app_state;

use ratatui::prelude::*;

use std::collections::HashMap;

pub fn state_of_episodes(
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
        if the_app.ui_state == the_types::UiState::State601KillingDownloads
            || the_app.ui_state == the_types::UiState::State001NewPodcastUrl
            || the_app.ui_state == the_types::UiState::State002NewPodcastName
        {
            episode_show::render_epi_list_empty(
                console_frame,
                elastic_epi_area,
                the_app,
                String::from(""),
                wait_color,
            );
        } else if the_app.ui_state == the_types::UiState::State103ShowEpisodes
            || the_app.ui_state == the_types::UiState::State401DownloadPaused
            || the_app.ui_state == the_types::UiState::State001NewPodcastUrl
            || the_app.ui_state == the_types::UiState::State002NewPodcastName
            || the_app.ui_state == the_types::UiState::State003ClickedNew
            || the_app.ui_state == the_types::UiState::State203DownloadingEvery
            || the_app.ui_state == the_types::UiState::State501Help
        {
            episode_show::render_epi_list(
                console_frame,
                elastic_epi_area,
                the_app,
                String::from("Select Episodes to Download"),
                wait_color,
            );
            episode_event::episode_vscroll(console_frame, elastic_epi_area, the_app);
        } else {
            episode_show::render_epi_list_empty(
                console_frame,
                elastic_epi_area,
                the_app,
                String::from(""),
                wait_color,
            );
        }
    }
}

pub fn colored_episodes(
    ordered_episodes: Vec<String>,
    local_episode_files: HashMap<String, String>,
    the_app: &mut app_state::DownApp,
    wait_color: Color,
) -> Vec<Line<'static>> {
    let mut the_waiting: HashMap<String, String> = HashMap::new();
    let selected_podcast = &the_app.selected_podcast;
    for (_sel_podcast, media_fname, _url_episode) in &the_app.download_deque {
        let full_epi_name = format!("{selected_podcast}/{media_fname}");
        the_waiting.insert(full_epi_name.to_string(), full_epi_name.to_string());
    }
    let text: Vec<Line> = ordered_episodes
        .into_iter()
        .enumerate()
        .map(|(episode_index, episode_name)| {
            let episode_hover_id = episode_event::get_episode_hover_index(episode_index, &the_app);
            if local_episode_files.contains_key(&episode_name) {
                let old_local_file = episode_colors::color_old_epi(&episode_name, wait_color);
                return Line::from(old_local_file);
            } else {
                let full_epi_name1 = format!("{selected_podcast}/{episode_name}");
                let num_bytes = g_current_active::my_get(full_epi_name1);
                let full_epi_name2 = format!("{selected_podcast}/{episode_name}");
                if num_bytes == "" {
                    return episode_colors::possible_or_waiting(
                        &the_waiting,
                        &full_epi_name2,
                        &episode_name,
                        wait_color,
                        episode_hover_id,
                    );
                } else {
                    return episode_colors::start_or_downloading_or_done(
                        &num_bytes,
                        &episode_name,
                        wait_color,
                    );
                }
            }
        })
        .collect();
    text
}
