#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::components::episodes::episode_acts;
use crate::components::episodes::episode_colors;
use crate::components::episodes::episode_display;
use crate::consts::const_colors;
use crate::consts::const_types;
use crate::globals::g_active;
use crate::state::state_app;

use ratatui::prelude::*;
use std::collections::HashMap;

pub fn case_state_of_episodes(
    console_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    app_dim: bool,
    is_downloading_paused: bool,
) {
    if the_app.selected_podcast != "" {
        let elastic_epi_area = episode_acts::rect_episode(console_frame);
        let mut wait_color = const_colors::NORMAL_BORDER_COL;
        if is_downloading_paused {
            wait_color = const_colors::PAUSE_COLOR;
        } else if app_dim {
            wait_color = const_colors::DIMMED_BACKGROUND_WAIT;
        }
        if the_app.ui_state == const_types::UiState::State601KillingDownloads
            || the_app.ui_state == const_types::UiState::State001NewPodcastUrl
            || the_app.ui_state == const_types::UiState::State002NewPodcastName
        {
            episode_display::display_display_render_epi_list_empty(
                console_frame,
                elastic_epi_area,
                the_app,
                wait_color,
            );
        } else if the_app.ui_state == const_types::UiState::State103ShowEpisodes
            || the_app.ui_state == const_types::UiState::State401DownloadPaused
            || the_app.ui_state == const_types::UiState::State001NewPodcastUrl
            || the_app.ui_state == const_types::UiState::State002NewPodcastName
            || the_app.ui_state == const_types::UiState::State003ClickedNew
            || the_app.ui_state == const_types::UiState::State203DownloadingEvery
            || the_app.ui_state == const_types::UiState::State501Help
        {
            episode_display::display_render_epi_list(
                console_frame,
                elastic_epi_area,
                the_app,
                String::from("Select Episodes to Download"),
                wait_color,
            );
            episode_acts::acts_episode_vscroll(console_frame, elastic_epi_area, the_app);
        } else {
            episode_display::display_display_render_epi_list_empty(
                console_frame,
                elastic_epi_area,
                the_app,
                wait_color,
            );
        }
    }
}

pub fn case_colored_episodes(
    ordered_episodes: Vec<String>,
    local_episode_files: HashMap<String, String>,
    the_app: &mut state_app::DownApp,
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
            let acts_episode_hover_id =
                episode_acts::acts_episode_hover_index(episode_index, &the_app);
            if local_episode_files.contains_key(&episode_name) {
                let old_local_file = episode_colors::colors_old_epi(&episode_name, wait_color);
                return Line::from(old_local_file);
            } else {
                let full_epi_name1 = format!("{selected_podcast}/{episode_name}");
                let num_bytes = g_active::active_bytes(full_epi_name1);
                let full_epi_name2 = format!("{selected_podcast}/{episode_name}");
                if num_bytes == "" {
                    return episode_colors::colors_possible_or_waiting(
                        &the_waiting,
                        &full_epi_name2,
                        &episode_name,
                        wait_color,
                        acts_episode_hover_id,
                    );
                } else {
                    return episode_colors::colors_start_down_done(
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
