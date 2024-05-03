#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::components::podcasts::podcast_contents;
use crate::consts::consts_types;
use crate::globals::g_active;
use crate::globals::g_stop;
use crate::media::media_chunks;
use crate::media::media_threads;
use crate::state::state_app;

pub fn reify_type(the_app: &mut state_app::DownApp) {
    if the_app.ui_state == consts_types::UiState::State101ReadingRss {
        let podcast_name = the_app.selected_podcast.clone();
        if podcast_name != "" {
            the_app.ui_state = consts_types::UiState::State102ShowWaiting;
        } else {
            the_app.ui_state = consts_types::UiState::StateNoFocus; // clicked on empty space
        }
    } else if the_app.ui_state == consts_types::UiState::State102ShowWaiting {
        the_app.local_episode_files = media_chunks::read_episode_dir(&the_app.selected_podcast);

        // what if no podcast !!!
        match podcast_contents::contents_episode_list(the_app) {
            Ok(_v) => {
                the_app.ui_state = consts_types::UiState::State103ShowEpisodes;
            }
            Err(e) => {
                the_app.ui_state = consts_types::UiState::State301WaitForPopErrorClose;
                the_app.cur_error = e.to_string();
            }
        };
    } else if the_app.ui_state == consts_types::UiState::State202SureEveryEpisode {
        let epi_2_url = the_app.episode_2_url.clone();
        let selected_podcast = the_app.selected_podcast.clone();
        for (media_name, file_url) in epi_2_url {
            let the_fname = media_name.clone();
            let the_url = file_url.clone();
            if !the_app.local_episode_files.contains_key(&the_fname) {
                media_threads::threads_queue(the_app, selected_podcast.clone(), the_fname, the_url);
            }
        }
        the_app.ui_state = consts_types::UiState::State203DownloadingEvery;
    } else if the_app.ui_state == consts_types::UiState::State601KillingDownloads {
        let num_downloading = g_active::active_downloading();
        if num_downloading == 0 {
            g_active::active_clear();
            the_app.download_deque.clear();
            g_stop::stop_false();
            the_app.ui_state = consts_types::UiState::State103ShowEpisodes;
        }
    }
}
