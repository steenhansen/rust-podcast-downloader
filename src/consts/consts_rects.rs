#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::consts_areas;

use ratatui::layout::Rect;
use ratatui::prelude::*;

// pub fn rect_ok_dialog22(console_frame: &mut Frame) -> Rect {
//     let area_frame = console_frame.size();
//     let area = rect_centered(60, 20, area_frame);

//     let mut close_err_area = area;
//     close_err_area.x = close_err_area.x + (close_err_area.width / 2) - 2;
//     close_err_area.y = close_err_area.y + (close_err_area.height / 2) - 1 + 2;
//     close_err_area.width = 6;
//     close_err_area.height = 3;
//     close_err_area
// }

pub fn rect_point_in(px: u16, py: u16, a_rect: Rect) -> bool {
    let l_side = a_rect.x;
    let r_side = a_rect.x + a_rect.width;
    let inside_hor = l_side <= px && px <= r_side;

    let t_side = a_rect.y;
    let b_side = a_rect.y + a_rect.height;
    let inside_ver = t_side <= py && py <= b_side;
    inside_hor && inside_ver
}

// pub fn rect_center_from_top(ok_size: Rect, centered_area: Rect) -> Rect {
//     let ok_x = centered_area.x + centered_area.width / 2 - ok_size.width / 2;
//     let ok_y = centered_area.y + ok_size.y;

//     let ok_area: ratatui::layout::Rect = Rect {
//         x: ok_x,
//         y: ok_y,
//         width: ok_size.width,
//         height: ok_size.height,
//     };
//     ok_area
// }

// pub fn rect_centered22(dialog_size: Rect, frame_area: Rect) -> Rect {
//     let middle_x = frame_area.width / 2 - dialog_size.width / 2;
//     let middle_y = frame_area.height / 2 - dialog_size.height / 2;

//     let centered_area: ratatui::layout::Rect = Rect {
//         x: middle_x,
//         y: middle_y,
//         width: dialog_size.width,
//         height: dialog_size.height,
//     };
//     centered_area
// }

pub fn rect_centered(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

pub fn rect_episode(console_frame: &mut Frame) -> Rect {
    let area_frame = console_frame.size();
    let elastic_episodes_area = Rect {
        x: consts_areas::START_X_EPISODE,
        y: consts_areas::START_Y_EPISODE,
        width: area_frame.width - 10,
        height: area_frame.height - 9,
    };
    elastic_episodes_area
}

pub fn rect_status(console_frame: &mut Frame) -> Rect {
    let area_frame = console_frame.size();
    let status_area = Rect {
        x: 0,
        y: area_frame.height - 1,
        width: area_frame.width,
        height: 1,
    };
    status_area
}

pub fn rect_title(console_frame: &mut Frame, the_title: &str) -> Rect {
    let area_frame = console_frame.size();
    let title_width = the_title.len() as u16;
    let left_start = (area_frame.width - title_width) / 2;
    let up_right_area = Rect {
        x: left_start,
        y: 0,
        width: title_width,
        height: 1,
    };
    up_right_area
}

pub fn rect_feed(console_frame: &mut Frame, the_url: &str) -> Rect {
    let area_frame = console_frame.size();
    let url_width = the_url.len() as u16;
    let mut left_start: i16 = area_frame.width as i16 - url_width as i16;
    if left_start < consts_areas::MIN_FEED_X_START {
        left_start = consts_areas::MIN_FEED_X_START;
    }
    let up_right_area = Rect {
        x: left_start as u16,
        y: area_frame.height - 1,
        width: url_width,
        height: 1,
    };
    up_right_area
}

// this is to be killed
pub fn rect_ok_dialog(console_frame: &mut Frame) -> Rect {
    let area_frame = console_frame.size();
    let area = rect_centered(60, 20, area_frame);

    let mut close_err_area = area;
    close_err_area.x = close_err_area.x + (close_err_area.width / 2) - 2;
    close_err_area.y = close_err_area.y + (close_err_area.height / 2) - 1 + 2;
    close_err_area.width = 6;
    close_err_area.height = 3;
    close_err_area
}

// pub fn rect_sure_yes(console_frame: &mut Frame) -> Rect {
//     let area_frame = console_frame.size();
//     let area = rect_centered(60, 41, area_frame);

//     let mut close_yes_area = area;
//     close_yes_area.x = close_yes_area.x + (close_yes_area.width / 2) - 3;
//     close_yes_area.y = close_yes_area.y + (close_yes_area.height / 2) - 1 + 2 - 4;
//     close_yes_area.width = 7;
//     close_yes_area.height = 3;
//     close_yes_area
// }

// pub fn rect_sure_no(console_frame: &mut Frame) -> Rect {
//     let area_frame = console_frame.size();
//     let area = rect_centered(60, 41, area_frame);

//     let mut close_no_area = area;
//     close_no_area.x = close_no_area.x + (close_no_area.width / 2) - 2 - 1;
//     close_no_area.y = close_no_area.y + (close_no_area.height / 2) - 1 + 2 + 1;
//     close_no_area.width = 6;
//     close_no_area.height = 3;
//     close_no_area
// }

// pub fn rect_sure_yes(console_frame: &mut Frame) -> Rect {
//     let area_frame = console_frame.size();
//     let area = rect_centered(60, 41, area_frame);

//     let mut close_yes_area = area;
//     close_yes_area.x = close_yes_area.x + (close_yes_area.width / 2) - 3;
//     close_yes_area.y = close_yes_area.y + (close_yes_area.height / 2) - 1 + 2 - 4;
//     close_yes_area.width = 7;
//     close_yes_area.height = 3;
//     close_yes_area
// }
