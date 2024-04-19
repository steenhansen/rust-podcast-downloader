pub const DEBUG_FILE: &str = "zzz_debug_log.txt";

pub const RSS_TEXT_FILE: &str = "podcast_url.rss";
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
