use crate::episodes_files;
use crate::podcast_files;
use crate::render_app;
use crate::the_types;
#[allow(unused)]
use log::{debug, info, trace, warn};

pub fn after_ui(the_app: &mut render_app::DownApp) {
    if the_app.ui_state == the_types::UiState::State101ReadingRss {
        the_app.ui_state = the_types::UiState::State102ShowWaiting;
    } else if the_app.ui_state == the_types::UiState::State102ShowWaiting {
        the_app.local_episode_files = episodes_files::read_episode_dir(&the_app.selected_podcast);
        let _reponse_x = match podcast_files::get_epi_list(the_app) {
            Ok(_v) => {
                the_app.ui_state = the_types::UiState::State103ShowEpisodes;
            }
            Err(_e) => {
                the_app.ui_state = the_types::UiState::StateWaitForPopErrorClose;
                the_app.cur_error = "no such url".to_string();
            }
        };
    }
}
