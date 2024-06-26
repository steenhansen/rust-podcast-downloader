#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_colors;
use crate::consts::const_globals;

use ratatui::prelude::*;
use std::collections::HashMap;

pub const CAN_DOWNLOAD_EPISODE: Color = Color::Green;
pub const WAITING_EPISODE: Color = Color::Green;
pub const JUST_GOT_EPISODE: Color = Color::Cyan;
pub const ACTIVE_EPISODE: Color = Color::Magenta;
pub const START_EPISODE_FG: Color = Color::Black;
pub const START_EPISODE_BG: Color = Color::White;
pub const OLD_LOCAL_EPISODE: Color = Color::DarkGray;

pub fn colors_start_down_done(
    num_bytes: &str,
    episode_name: &str,
    wait_color: Color,
) -> Line<'static> {
    if num_bytes == const_globals::DOWN_BYTES_FINISHED {
        let new_local_file = colors_finished_epi(&episode_name, wait_color);
        return Line::from(new_local_file);
    } else {
        if num_bytes == "0" {
            let await_start = colors_start_epi(&episode_name, wait_color);
            return Line::from(await_start);
        } else {
            let downloading_file = colors_continue_epi(num_bytes, &episode_name, wait_color);
            return Line::from(downloading_file);
        }
    }
}

pub fn colors_possible_or_waiting(
    the_waiting: &HashMap<String, String>,
    full_epi_name: &str,
    episode_name: &str,
    wait_color: Color,
    acts_episode_hover_id: i32,
) -> Line<'static> {
    if the_waiting.contains_key(full_epi_name) {
        let indented_waiting = const_globals::WAITING_FINISHED_INDENT.to_owned() + episode_name;
        let waiting_file = colors_waiting_epi(&indented_waiting, const_colors::ON_WAITLIST);
        return Line::from(waiting_file);
    } else if acts_episode_hover_id >= 0 {
        let hover_file = colors_hover_epi(&episode_name);
        return Line::from(hover_file);
    } else {
        let possible_file = colors_possible_epi(&episode_name, wait_color);
        return Line::from(possible_file);
    }
}

pub fn colors_hover_epi(episode_name: &str) -> Span<'static> {
    let waiting_file = Span::styled(
        episode_name.to_owned(),
        Style::default()
            .fg(const_colors::AN_EPISODE_TEXT_HOVER)
            .bg(const_colors::AN_EPISODE_BACK_HOVER),
    );
    waiting_file
}

pub fn colors_old_epi(episode_name: &str, wait_color: Color) -> Span<'static> {
    let old_have_color = match wait_color {
        Color::Reset => OLD_LOCAL_EPISODE,
        _ => wait_color,
    };
    let indented_old = const_globals::WAITING_FINISHED_INDENT.to_owned() + episode_name;
    let old_local_file = Span::styled(indented_old, Style::default().fg(old_have_color));
    old_local_file
}

pub fn colors_finished_epi(episode_name: &str, wait_color: Color) -> Span<'static> {
    let finished_color = match wait_color {
        Color::Reset => JUST_GOT_EPISODE,
        _ => wait_color,
    };
    let just_finished = const_globals::WAITING_FINISHED_INDENT.to_owned() + episode_name;
    let new_local_file = Span::styled(just_finished, Style::default().fg(finished_color));
    new_local_file
}

pub fn colors_start_epi(episode_name: &str, wait_color: Color) -> Span<'static> {
    let start_text_color = match wait_color {
        Color::Reset => START_EPISODE_FG,
        _ => wait_color,
    };
    let start_back_color = match wait_color {
        Color::Reset => START_EPISODE_BG,
        _ => const_colors::BLACK_WAIT,
    };
    let new_f_pos = const_globals::DOWN_BYTES_START_SHOW.to_owned() + " - " + episode_name;
    let new_local_file = Span::styled(
        new_f_pos,
        Style::default().fg(start_text_color).bg(start_back_color),
    );
    new_local_file
}

pub fn colors_continue_epi(
    num_bytes: &str,
    episode_name: &str,
    wait_color: Color,
) -> Span<'static> {
    let continue_color = match wait_color {
        Color::Reset => ACTIVE_EPISODE,
        _ => wait_color,
    };
    let new_f_pos = num_bytes.to_owned() + " - " + episode_name;
    let new_local_file = Span::styled(new_f_pos, Style::default().fg(continue_color));
    new_local_file
}

pub fn colors_waiting_epi(episode_name: &str, wait_color: Color) -> Span<'static> {
    let waiting_color = match wait_color {
        Color::Reset => WAITING_EPISODE,
        _ => wait_color,
    };

    let waiting_file = Span::styled(episode_name.to_owned(), Style::default().fg(waiting_color));
    waiting_file
}

pub fn colors_possible_epi(episode_name: &str, wait_color: Color) -> Span<'static> {
    let possible_color = match wait_color {
        Color::Reset => CAN_DOWNLOAD_EPISODE,
        _ => wait_color,
    };
    let possible_file = Span::styled(episode_name.to_owned(), Style::default().fg(possible_color));
    possible_file
}
