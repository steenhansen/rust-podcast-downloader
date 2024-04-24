#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::app_state;
use crate::episode_threads;
use crate::episodes_chunks;
use crate::podcast_files;
use crate::the_types;

pub fn after_ui(the_app: &mut app_state::DownApp) {
    if the_app.ui_state == the_types::UiState::State101ReadingRss {
        the_app.ui_state = the_types::UiState::State102ShowWaiting;
    } else if the_app.ui_state == the_types::UiState::State102ShowWaiting {
        the_app.local_episode_files = episodes_chunks::read_episode_dir(&the_app.selected_podcast);
        match podcast_files::get_epi_list(the_app) {
            Ok(_v) => {
                the_app.ui_state = the_types::UiState::State103ShowEpisodes;
            }
            Err(e) => {
                the_app.ui_state = the_types::UiState::State301WaitForPopErrorClose;
                the_app.cur_error = e.to_string();
            }
        };
    } else if the_app.ui_state == the_types::UiState::State202SureAllEpisodes {
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
        the_app.ui_state = the_types::UiState::State103ShowEpisodes;
    }
}
