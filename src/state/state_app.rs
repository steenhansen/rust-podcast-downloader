#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_types;
use crate::consts::const_types::*;

use ratatui::widgets::*;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Default, Debug)]
pub struct DownApp {
    // when adding a new podcast
    pub new_podcast_name: PodcastName, // 'NASA Image of the Day'
    pub new_podcast_url: PodcastUrl,   // https://www.nasa.gov/feeds/iotd-feed

    // podcasts
    pub selected_podcast: PodcastName, //  "NASA Image of the Day"
    pub ordered_podcasts: Vec<PodcastName>, // ["NASA Image of the Day",
    pub init_erased_dirs: HashMap<PodcastName, bool>, // {"./NASA Image of the Day": true,

    // episodes
    pub local_episode_files: HashMap<EpisodeFilename, EpisodeFilename>, // {"0019_NGC-4254-Webb-Image.png": "0019_NGC-4254-Webb-Image.png",
    pub episode_2_url: HashMap<EpisodeFilename, EpisodeUrl>, //       {"0019_NGC-4254-Webb-Image.png": "https://www.nasa.gov/wp-content/uploads/2024/02/53495965723-5c5c821f78-o.png",
    pub episode_2_len: HashMap<EpisodeFilename, EpisodeLength>, //       {"0019_NGC-4254-Webb-Image.png": 4243566,
    pub ordered_episodes: Vec<EpisodeFilename>,                 // ["0019_NGC-4254-Webb-Image.png",
    pub download_deque: VecDeque<PodcastMetadataTuple>, // [("NASA Image of the Day", "0019_NGC-4254-Webb-Image.png", "https://www.nasa.gov/wp-content/uploads/2024/02/53495965723-5c5c821f78-o.png"),

    // scrolling up and down
    pub scrolled_podcasts_pos: usize, // 2   - these are private in ScrollbarState so must maintain
    pub scrolled_episodes_pos: usize, // 61
    pub state_scroll_podcasts: ScrollbarState, // { content_length: 1, position: 0, viewport_content_length: 0 }
    pub state_scroll_episodes: ScrollbarState, // { content_length: 60, position: 21, viewport_content_length: 0 }

    pub cur_error: String,  //  Error - "http://www.xe.com" is not valid XML
    pub fast_med_slow: u16, // 0,1,2

    pub ui_state: UiState, // State103ShowEpisodes

    pub hover_element: String, // BTN-ALL
    pub acts_episode_hover_row: usize,
    pub happening_podcast_hover_row: usize,
    pub pause_type_103_203: UiState, // when pause either State103ShowEpisodes or State203DownloadingEvery
    pub pause_help: UiState,
}

//
//let s = HashMap::from([(1, 2), (3, 4)]);

pub const HOVER_NONE: &str = "HOVER-NONE";

pub const HOVER_BTN_NEW: &str = "HOVER-BTN-NEW";
pub const HOVER_BTN_EVERY: &str = "HOVER-BTN-EVERY";
pub const HOVER_BTN_STOP: &str = "HOVER-BTN-KILL";

pub const HOVER_X_QUIT: &str = "HOVER-X-QUIT";

pub const HOVER_PAUSE: &str = "HOVER-PAUSE";
pub const HOVER_RESOURCES: &str = "HOVER-RESOURCES";
pub const HOVER_NEW_URL: &str = "HOVER-NEW-URL";
pub const HOVER_NEW_NAME: &str = "HOVER-NEW-NAME";

pub const HOVER_EPISODES: &str = "HOVER-EPISODES";
pub const HOVER_PODCASTS: &str = "HOVER-PODCASTS";

pub const HOVER_HELP_DIALOG: &str = "HOVER-HELP-DIALOG";
pub const HOVER_HELP_OK: &str = "HOVER-HELP-OK";

pub const HOVER_ERROR_DIALOG: &str = "HOVER-ERROR-DIALOG";
pub const HOVER_ERROR_OK: &str = "HOVER-ERROR-OK";

pub const HOVER_SURE_DIALOG: &str = "HOVER-SURE-DIALOG";
pub const HOVER_SURE_YES: &str = "HOVER-SURE-YES";
pub const HOVER_SURE_NO: &str = "HOVER-SURE-NO";

pub fn app_dim(the_app: &DownApp) -> bool {
    let ui_state = the_app.ui_state;
    if ui_state == const_types::UiState::State101ReadingRss
        || ui_state == const_types::UiState::State102ShowWaiting
        || ui_state == const_types::UiState::State201EveryEpisode
        || ui_state == const_types::UiState::State202SureEveryEpisode
        || ui_state == const_types::UiState::State301WaitForPopErrorClose
        || ui_state == const_types::UiState::State501Help
    {
        return true;
    }
    false
}
