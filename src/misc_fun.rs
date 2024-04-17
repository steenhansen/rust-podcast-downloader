//             //https://docs.rs/ratatui/latest/src/tabs/tabs.rs.html#144
#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::const_areas;
use crate::const_globals;
use crossterm::event::{
    KeyCode, KeyEvent, KeyModifiers, MouseButton::Left, MouseEvent, MouseEventKind,
};
use ratatui::layout::Rect;
use ratatui::prelude::*;
use regex::Regex;

pub fn point_in_rect(px: u16, py: u16, a_rect: Rect) -> bool {
    let l_side = a_rect.x;
    let r_side = a_rect.x + a_rect.width;
    let inside_hor = l_side <= px && px <= r_side;

    let t_side = a_rect.y;
    let b_side = a_rect.y + a_rect.height;
    let inside_ver = t_side <= py && py <= b_side;

    inside_hor && inside_ver
}

pub fn mouse_click(mouse_event: MouseEvent) -> bool {
    if mouse_event.kind == MouseEventKind::Down(Left) {
        return true;
    }
    false
}
pub fn mouse_scroll(mouse_event: MouseEvent) -> bool {
    if mouse_event.kind == MouseEventKind::ScrollUp {
        return true;
    }
    if mouse_event.kind == MouseEventKind::ScrollDown {
        return true;
    }
    false
}
pub fn is_control_c(key_event: KeyEvent) -> bool {
    let is_contorl = key_event.modifiers == KeyModifiers::CONTROL;
    let is_upper_c = key_event.code == KeyCode::Char('C');
    let is_lower_c = key_event.code == KeyCode::Char('c');
    if is_contorl && (is_upper_c || is_lower_c) {
        return true;
    }
    false
}

pub fn below_podcasts(
    mouse_event: MouseEvent,
    scrolled_podcasts: usize,
    sorted_len: usize,
) -> bool {
    let row = mouse_event.row as usize;
    let visible_podcasts = sorted_len - scrolled_podcasts;
    let last_podcast_y = const_areas::START_Y_PODCAST_US + visible_podcasts;
    if row > last_podcast_y {
        return true;
    }
    false
}

pub fn below_episodes(
    mouse_event: MouseEvent,
    scrolled_episodes: usize,
    sorted_len: usize,
) -> bool {
    let row = mouse_event.row as usize;
    let visible_episodes = sorted_len - scrolled_episodes;
    let last_episode_y = const_areas::START_Y_EPISODE_US + visible_episodes;
    if row > last_episode_y {
        return true;
    }
    false
}

pub fn clean_title(actual_title: String) -> String {
    let only_alpha = Regex::new(const_globals::ONLY_ALPHA_REGX).unwrap();
    let multi_spaces = Regex::new(const_globals::MULTI_SPACES_REGX).unwrap();

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

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
