#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::chunks::episode_threads;
use crate::chunks::episodes_chunks;
use crate::globals::g_current_active;
use crate::globals::g_stop_io;
use crate::podcasts::podcast_episodes;
use crate::state::app_state;

use crate::consts::the_types;

pub fn change_app_state_type(the_app: &mut app_state::DownApp) {
    if the_app.ui_state == the_types::UiState::State101ReadingRss {
        let podcast_name = the_app.selected_podcast.clone();
        if podcast_name != "" {
            the_app.ui_state = the_types::UiState::State102ShowWaiting;
        } else {
            the_app.ui_state = the_types::UiState::StateNoFocus; // clicked on empty space
        }
    } else if the_app.ui_state == the_types::UiState::State102ShowWaiting {
        the_app.local_episode_files = episodes_chunks::read_episode_dir(&the_app.selected_podcast);

        // what if no podcast !!!
        match podcast_episodes::get_epi_list(the_app) {
            Ok(_v) => {
                the_app.ui_state = the_types::UiState::State103ShowEpisodes;
            }
            Err(e) => {
                the_app.ui_state = the_types::UiState::State301WaitForPopErrorClose;
                the_app.cur_error = e.to_string();
            }
        };
    } else if the_app.ui_state == the_types::UiState::State202SureEveryEpisode {
        let epi_2_url = the_app.episode_2_url.clone();
        let selected_podcast = the_app.selected_podcast.clone();
        for (media_name, file_url) in epi_2_url {
            let the_fname = media_name.clone();
            let the_url = file_url.clone();
            if !the_app.local_episode_files.contains_key(&the_fname) {
                episode_threads::queue_episode_download(
                    the_app,
                    selected_podcast.clone(),
                    the_fname,
                    the_url,
                );
            }
        }
        the_app.ui_state = the_types::UiState::State203DownloadingEvery;
    } else if the_app.ui_state == the_types::UiState::State601KillingDownloads {
        let num_downloading = g_current_active::active_downloading();
        if num_downloading == 0 {
            g_current_active::new_pod_clear();
            the_app.download_deque.clear();
            g_stop_io::stop_stoping();
            the_app.ui_state = the_types::UiState::State103ShowEpisodes;
        }
    }
}
