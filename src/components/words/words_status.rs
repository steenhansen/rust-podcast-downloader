#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_colors;
use crate::globals::g_active;
use crate::state::state_app;

use ratatui::layout::Rect;
use ratatui::{prelude::*, widgets::*};

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

pub fn status_show(
    console_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    app_dim: bool,
    is_downloading_paused: bool,
) {
    let status_area = rect_status(console_frame);
    let mut wait_color = const_colors::NORMAL_BORDER_COL;
    if is_downloading_paused {
        wait_color = const_colors::PAUSE_COLOR;
    } else if app_dim {
        wait_color = const_colors::DIMMED_BACKGROUND_WAIT;
    }
    let num_downloading = g_active::active_downloading();
    let num_waiting = the_app.download_deque.len();
    let stat_mess = format!(
        " Active Files {:?}, Waiting {:?}",
        num_downloading, num_waiting
    );
    status_render(console_frame, status_area, the_app, &stat_mess, wait_color);
}

pub fn status_render(
    console_frame: &mut Frame,
    draw_area: Rect,
    _the_app: &mut state_app::DownApp,
    box_title: &str,
    status_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let title = Block::new()
        .title(box_title)
        .style(Style::default().fg(status_color));
    console_frame.render_widget(title, area_safe);
}
