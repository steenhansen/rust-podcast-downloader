#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_globals;

use regex::Regex;

pub fn epi_prefix_num(episode_index: usize) -> String {
    let str_index = format!(
        "{:0prefix_digits$}",
        episode_index,
        prefix_digits = const_globals::PREFIX_NUM_DIGITS
    );
    str_index //0001_first_podcast
}

pub fn clean_title(actual_title: String) -> String {
    let only_alpha = Regex::new(const_globals::ONLY_ALPHA_REGX).expect("only-alpha-regex-err");
    let multi_spaces =
        Regex::new(const_globals::MULTI_SPACES_REGX).expect("multi-spaces-regex-err");

    let title_change = actual_title.as_str();
    let title_cow = only_alpha.replace_all(title_change, const_globals::REPLACE_TO_SPACE);
    let spaced_title = title_cow.to_string();
    let trimmed_title = spaced_title.trim();
    let multi_spaces = multi_spaces.replace_all(trimmed_title, const_globals::REPLACE_TO_SPACE);
    let solo_blank_title = multi_spaces.to_string();
    let dashed_title = solo_blank_title.replace(" ", "-");
    dashed_title
}

pub fn file_type_real(an_url: String) -> String {
    let v: Vec<&str> = an_url.split('?').collect();
    let first_part = v[0];
    if first_part.ends_with(".jpg") {
        return "jpg".to_string();
    }
    if first_part.ends_with(".jpeg") {
        return "jpeg".to_string();
    }
    if first_part.ends_with(".png") {
        return "png".to_string();
    }
    if first_part.ends_with(".webp") {
        return "webp".to_string();
    }
    if first_part.ends_with(".pdf") {
        return "pdf".to_string();
    }
    if first_part.ends_with(".mp3") {
        return "mp3".to_string();
    }
    return "unknown".to_string();
}
