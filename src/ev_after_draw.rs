use crate::app_state;
//use crate::close_error;
use crate::const_globals;
use crate::episode_threads;
use crate::episodes_files;
use crate::g_current_active;
use crate::podcast_files;
use crate::the_types;
#[allow(unused)]
use log::{debug, info, trace, warn};
use std::thread;
use std::time::Duration;
pub fn after_ui(the_app: &mut app_state::DownApp) {
    if the_app.ui_state == the_types::UiState::State101ReadingRss {
        the_app.ui_state = the_types::UiState::State102ShowWaiting;
    } else if the_app.ui_state == the_types::UiState::State102ShowWaiting {
        the_app.local_episode_files = episodes_files::read_episode_dir(&the_app.selected_podcast);
        match podcast_files::get_epi_list(the_app) {
            Ok(_v) => {
                the_app.ui_state = the_types::UiState::State103ShowEpisodes;
            }
            Err(e) => {
                the_app.ui_state = the_types::UiState::StateWaitForPopErrorClose;
                the_app.cur_error = e.to_string(); //"Not a RSS file ato such url".to_string();
            }
        };
    } else if the_app.ui_state == the_types::UiState::State202SureAllEpisodes {
        let epi_2_url = the_app.episode_2_url.clone();
        let selected_podcast = the_app.selected_podcast.clone();
        for (media_name, file_url) in epi_2_url {
            let the_fname = media_name.clone();
            let the_url = file_url.clone();
            if !the_app.local_episode_files.contains_key(&the_fname) {
                //  warn!(" enque ev_after_draw, after_ui {:?}", the_fname);
                episode_threads::queue_episode_download(
                    the_app,
                    selected_podcast.clone(),
                    the_fname,
                    the_url,
                );
            }
        }

        the_app.ui_state = the_types::UiState::State103ShowEpisodes;
    } else if the_app.ui_state == the_types::UiState::State203DownloadedingAll {
        // let num_downloading = g_current_active::active_downloading();
        // //warn!("in downloading iall = {:?}", num_downloading);
        // if num_downloading == 0 {
        //     // warn!(
        //     //     "************ 000000 ***** {:?}",
        //     //     g_current_active::G_CURRENT_ACTIVE
        //     // );
        //     the_app.ui_state = the_types::UiState::State204AfterAll;
        // }
        the_app.ui_state = the_types::UiState::State103ShowEpisodes;
    } else if the_app.ui_state == the_types::UiState::State204AfterAll {
        // let num_downloading = g_current_active::active_downloading();
        // warn!("in downloading iall = {:?}", num_downloading);
        // if num_downloading == 0 {
        //     warn!("***************** {:?}", g_current_active::G_CURRENT_ACTIVE);
        //     // we need to see the finished things !!

        // warn!(
        //     " 0111111111111110 the read {:?}",
        //     the_app.local_episode_files.len()
        // );
        // thread::sleep(Duration::from_secs(const_globals::SLEEP_SEC_SLOW));
        // warn!(" 11 the read {:?}", the_app.local_episode_files.len());
        // the_app.local_episode_files = episodes_files::read_episode_dir(&the_app.selected_podcast);
        //   warn!(" 22 the read {:?}", the_app.local_episode_files.len());

        // the_app.local_episode_files =
        //    // episodes_files::read_episode_dir(&the_app.selected_podcast);
        //             the_app.local_episode_files = episodes_files::read_episode_dir(&the_app.selected_podcast);
        //warn!("in chagne ");
        the_app.ui_state = the_types::UiState::State103ShowEpisodes;
    }
    //
}
