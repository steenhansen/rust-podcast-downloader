use crate::the_types::*;

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
}
