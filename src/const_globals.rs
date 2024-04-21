use ratatui::prelude::*;
use std::time::Duration;

pub const DEBUG_FILE: &str = "zzz_debug_log.txt";

pub const RSS_TEXT_FILE: &str = "podcast_address.rss";
pub const INT_PREFIX_Y_N: &str = "podcast_prefix.int";

pub const REPLACE_TO_SPACE: &str = " ";
pub const WORKING_FILE: &str = "-working";
pub const LEADING_0_SIZE: usize = 4;

pub const ROOT_DIR: &str = "./";

pub const DOWNLOADED_MEDIA: &str = "-1234";

pub const ONLY_ALPHA_REGX: &str = r"[^a-zA-Z0-9 -]+";
pub const MULTI_SPACES_REGX: &str = r"\s\s+";

// to find images when there is no enclosure
pub const FIND_PICTURES: &str = r####"=('|")[^"]*(.jpg|.jpeg|.png|.png|.webp)("|')"####;

pub const RADIO_RESOURCES: [&'static str; 3] = ["Fast", "Medium", "Slow"];

pub const _SLEEP_SEC_FAST: u64 = 0;
pub const SLEEP_SEC_MED: u64 = 2;
pub const SLEEP_SEC_SLOW: u64 = 10;

pub const MAX_SPAWNS_FAST: usize = 8;
pub const MAX_SPAWNS_MED: usize = 3;
pub const MAX_SPAWNS_SLOW: usize = 1;

pub const OLD_LOCAL_EPISODE: Color = Color::DarkGray;

pub const JUST_GOT_EPISODE: Color = Color::Cyan;

pub const CAN_DOWNLOAD_EPISODE: Color = Color::Green;
pub const WAITING_EPISODE: Color = Color::Yellow;
pub const ACTIVE_EPISODE: Color = Color::White;

pub const START_EPISODE_FG: Color = Color::Black;
pub const START_EPISODE_BG: Color = Color::White;

pub const CHUNK_TIMEOUT: Duration = Duration::from_secs(100);
pub const CHUNK_SIZE: u32 = 1_000_000;

pub const RSS_SOME_TIMEOUT: Option<Duration> = Some(Duration::from_secs(100));
