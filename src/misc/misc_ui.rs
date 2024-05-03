#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::consts_areas;
use ratatui::layout::Rect;

pub fn rect_point_in(px: u16, py: u16, a_rect: Rect) -> bool {
    let l_side = a_rect.x;
    let r_side = a_rect.x + a_rect.width;
    let inside_hor = l_side <= px && px <= r_side;

    let t_side = a_rect.y;
    let b_side = a_rect.y + a_rect.height;
    let inside_ver = t_side <= py && py <= b_side;
    inside_hor && inside_ver
}

use crossterm::event::{
    KeyCode, KeyEvent, KeyModifiers, MouseButton::Left, MouseEvent, MouseEventKind,
};

pub fn left_click(kind_click: MouseEventKind) -> bool {
    if kind_click == MouseEventKind::Down(Left) || kind_click == MouseEventKind::Up(Left) {
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

pub fn mouse_click(mouse_event: MouseEvent) -> bool {
    if mouse_event.kind == MouseEventKind::Down(Left) {
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
    scrolled_podcasts_pos: usize,
    sorted_len: usize,
) -> bool {
    let row = mouse_event.row as usize;
    let visible_podcasts = sorted_len - scrolled_podcasts_pos;

    let chunk_start_y_podcast: usize = consts_areas::START_Y_PODCAST as usize;

    let last_podcast_y = chunk_start_y_podcast + visible_podcasts;
    if row > last_podcast_y {
        return true;
    }
    false
}

pub fn below_episodes(
    mouse_event: MouseEvent,
    scrolled_episodes_pos: usize,
    sorted_len: usize,
) -> bool {
    let row = mouse_event.row as usize;
    let visible_episodes = sorted_len - scrolled_episodes_pos;

    let chunk_start_y_episode: usize = consts_areas::START_Y_EPISODE as usize;

    let last_episode_y = chunk_start_y_episode + visible_episodes;
    if row > last_episode_y {
        return true;
    }
    false
}
