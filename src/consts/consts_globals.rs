use ratatui::prelude::*;
use std::time::Duration;

pub const PREFIX_NUM_DIGITS: usize = 4;

pub const WAITING_FINISHED_INDENT: &str = "     ";

pub const DOWN_BYTES_FINISHED: &str = "DOWN-BYTES-FINISHED";
pub const DOWN_BYTES_STOPPED: &str = "DOWN-BYTES-KILLED";

pub const DOWN_BYTES_INITL_SHOW: &str = "       ";
pub const DOWN_BYTES_START_SHOW: &str = "0000000";

/////////////////

pub const DEBUG_FILE: &str = "debug_log.txt";

pub const RSS_TEXT_FILE: &str = "podcast_address.rss";

pub const REPLACE_TO_SPACE: &str = " ";

pub const ROOT_DIR: &str = "./";

pub const ONLY_ALPHA_REGX: &str = r"[^a-zA-Z0-9 -]+";
pub const MULTI_SPACES_REGX: &str = r"\s\s+";

// to find images when there is no enclosure
pub const FIND_PICTURES: &str = r####"=('|")[^"]*(.jpg|.jpeg|.png|.png|.webp)("|')"####;

//pub const CLEAR_SCREEN: &str = "\x1B[2J\x1B[1;1H";
pub const CLEAR_SCREEN: &str = "the-end"; // hides errors

pub const RADIO_RESOURCES: [&'static str; 3] = ["Fast", "Medium", "Slow"];

pub const _SLEEP_SEC_FAST: u64 = 0;
pub const SLEEP_SEC_MED: u64 = 2;
pub const SLEEP_SEC_SLOW: u64 = 10;

pub const PAUSE_SLEEP_SEC: u64 = 1;

pub const MAX_SPAWNS_FAST: usize = 8;
pub const MAX_SPAWNS_MED: usize = 3;
pub const MAX_SPAWNS_SLOW: usize = 1;

pub const OLD_LOCAL_EPISODE: Color = Color::DarkGray;

pub const JUST_GOT_EPISODE: Color = Color::Cyan;

pub const NO_EPISODES_HOVER: i32 = -1;

pub const NO_PODCASTS_HOVER: i32 = -1;

pub const CAN_DOWNLOAD_EPISODE: Color = Color::Green;
pub const WAITING_EPISODE: Color = Color::Green;

pub const ACTIVE_EPISODE: Color = Color::White;

pub const START_EPISODE_FG: Color = Color::Black;
pub const START_EPISODE_BG: Color = Color::White;

pub const CHUNK_TIMEOUT: Duration = Duration::from_secs(100);
pub const CHUNK_SIZE: u32 = 1_000_000;

pub const RSS_SOME_TIMEOUT: Option<Duration> = Some(Duration::from_secs(100));

pub const PODCAST_SELECTED: Color = Color::White;
pub const PODCAST_NOT_SELECTED: Color = Color::DarkGray;

pub const WORKING_FILE: &str = "-working";

pub const PAUSE_COLOR: Color = Color::Yellow;

pub const PAUSE_BACK_COLOR: Color = Color::Yellow;
pub const PAUSE_TEXT_COLOR: Color = Color::Black;

pub const DIMMED_BACKGROUND_WAIT: Color = Color::DarkGray;
pub const BLACK_WAIT: Color = Color::Black;

pub const NORMAL_BORDER_COL: Color = Color::Reset;

pub const TITLE_COLOR: Color = Color::White;

pub const BTN_EVERY_BACK_HOVER: Color = Color::LightGreen;
pub const BTN_EVERY_BACK_READY: Color = Color::Green;
pub const BTN_EVERY_BACK_DEAD: Color = Color::DarkGray;

pub const BTN_EVERY_TEXT_HOVER: Color = Color::Black;
pub const BTN_EVERY_TEXT_READY: Color = Color::White;
pub const BTN_EVERY_TEXT_DEAD: Color = Color::Black;

pub const BTN_X_BORDER_HOVER: Color = Color::White;
pub const BTN_X_BORDER_READY: Color = Color::Gray;

pub const PAUSE_HOVER: Color = Color::White;
pub const PAUSE_READY: Color = Color::Gray;
pub const PAUSE_DEAD: Color = Color::DarkGray;

pub const RESOURCE_HOVER: Color = Color::White;
pub const RESOURCE_READY: Color = Color::Gray;

pub const INPUT_TEXT_HOVER: Color = Color::White;
pub const INPUT_TEXT_READY: Color = Color::Gray;

pub const EPISODES_HOVER: Color = Color::White;
pub const EPISODES_READY: Color = Color::Gray;

pub const AN_EPISODE_BACK_HOVER: Color = Color::White;
pub const AN_EPISODE_TEXT_HOVER: Color = Color::Black;

pub const EPISODE_LOADING: Color = Color::Red;

pub const PODCASTS_HOVER: Color = Color::White;
pub const PODCASTS_READY: Color = Color::Gray;

pub const A_PODCAST_BACK_HOVER: Color = Color::White;
pub const A_PODCAST_TEXT_HOVER: Color = Color::Black;

pub const BYTE_COUNT_INIT: u32 = 0;
pub const BYTE_COUNT_START: u32 = 1;

pub const RESOURCE_FAST: u16 = 0;
pub const RESOURCE_MED: u16 = 1;
pub const RESOURCE_SLOW: u16 = 2;

pub const ON_WAITLIST: Color = Color::Blue;

pub const BLANK_PODCAST: &str = "";

pub const PAGE_SIZE: usize = 25;

pub const FOCUS_HOVER: Color = Color::White;
pub const FOCUS_NOT: Color = Color::Gray;
