use std::time::Duration;

pub const DEBUG_FILE: &str = "DEBUG_UI.log";

pub const RSS_TEXT_FILE: &str = "zzzz-podcast_address.rss";
pub const MEDIA_TEXT_FILE: &str = "zzzz-podcast_type.rss";

pub const BYTE_COUNT_INIT: u32 = 0;
pub const BYTE_COUNT_START: u32 = 1;

pub const RESOURCE_FAST: u16 = 0;
pub const RESOURCE_MED: u16 = 1;
pub const RESOURCE_SLOW: u16 = 2;

pub const BLANK_PODCAST: &str = "";

pub const PAGE_SIZE: usize = 25;

pub const PREFIX_NUM_DIGITS: usize = 4;

pub const WAITING_FINISHED_INDENT: &str = "     ";

pub const DOWN_BYTES_FINISHED: &str = "DOWN-BYTES-FINISHED";
pub const DOWN_BYTES_STOPPED: &str = "DOWN-BYTES-KILLED";

pub const DOWN_BYTES_INITL_SHOW: &str = "       ";
pub const DOWN_BYTES_START_SHOW: &str = "0000000";

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

pub const NO_EPISODES_HOVER: i32 = -1;

pub const NO_PODCASTS_HOVER: i32 = -1;

pub const CHUNK_TIMEOUT: Duration = Duration::from_secs(100);
pub const CHUNK_SIZE: u32 = 1_000_000;

pub const RSS_SOME_TIMEOUT: Option<Duration> = Some(Duration::from_secs(3));

pub const WORKING_FILE: &str = "-working";
