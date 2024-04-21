use crate::areas_consts;

#[allow(unused)]
use log::{debug, info, trace, warn};
use ratatui::layout::Rect;
use ratatui::prelude::*;

pub fn point_in_rect(px: u16, py: u16, a_rect: Rect) -> bool {
    let l_side = a_rect.x;
    let r_side = a_rect.x + a_rect.width;
    let inside_hor = l_side <= px && px <= r_side;

    let t_side = a_rect.y;
    let b_side = a_rect.y + a_rect.height;
    let inside_ver = t_side <= py && py <= b_side;

    inside_hor && inside_ver
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let center_layout = Layout::vertical([
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
    .split(center_layout[1])[1]
}

pub fn get_error_close_area(console_frame: &mut Frame) -> Rect {
    let area_frame = console_frame.size();
    let area = centered_rect(60, 20, area_frame);

    let mut close_err_area = area;
    close_err_area.x = close_err_area.x + (close_err_area.width / 2) - 2;
    close_err_area.y = close_err_area.y + (close_err_area.height / 2) - 1 + 1;
    close_err_area.width = 5;
    close_err_area.height = 3;
    close_err_area
}

pub fn get_quit_area(console_frame: &mut Frame) -> Rect {
    let area_frame = console_frame.size();
    let up_right_area = Rect {
        x: area_frame.width - 5,
        y: 0,
        width: 5,
        height: 3,
    };
    up_right_area
}

pub fn get_status_area(console_frame: &mut Frame) -> Rect {
    let area_frame = console_frame.size();
    let status_area = Rect {
        x: 0,
        y: area_frame.height - 1,
        width: area_frame.width / 2,
        height: 1,
    };
    status_area
}

pub fn get_episode_area(console_frame: &mut Frame) -> Rect {
    let area_frame = console_frame.size();
    let elastic_episodes_area = Rect {
        x: areas_consts::START_X_EPISODE,
        y: areas_consts::START_Y_EPISODE,
        width: area_frame.width - 10,
        height: area_frame.height - 9,
    };
    elastic_episodes_area
}

pub fn get_podcast_area(console_frame: &mut Frame) -> Rect {
    let area_frame = console_frame.size();
    let elastic_podcasts_area = Rect {
        x: areas_consts::START_X_PODCAST,
        y: areas_consts::START_Y_PODCAST,
        width: areas_consts::WIDTH_PODCAST,
        height: area_frame.height - 9,
    };
    elastic_podcasts_area
}
