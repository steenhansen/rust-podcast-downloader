#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::area_rects;
use crate::state::app_state;

use crate::consts::const_globals;
use crate::globals::g_current_active;
use ratatui::layout::Rect;

use ratatui::{prelude::*, widgets::*};

pub fn show_status(
    console_frame: &mut Frame,
    the_app: &mut app_state::DownApp,
    dim_background: bool,
    is_downloading_paused: bool,
) {
    let status_area = area_rects::get_status_area(console_frame);
    let mut wait_color = const_globals::NORMAL_BORDER_COL;
    if is_downloading_paused {
        wait_color = const_globals::PAUSE_COLOR;
    } else if dim_background {
        wait_color = const_globals::DIMMED_BACKGROUND_WAIT;
    }
    let num_downloading = g_current_active::active_downloading();
    let num_waiting = the_app.download_deque.len();
    let stat_mess = format!(
        " Active Files {:?}, Waiting {:?}",
        num_downloading, num_waiting
    );
    render_status(console_frame, status_area, the_app, &stat_mess, wait_color);
}

pub fn render_status(
    console_frame: &mut Frame,
    draw_area: Rect,
    _the_app: &mut app_state::DownApp,
    box_title: &str,
    status_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let title = Block::new()
        .title(box_title)
        .style(Style::default().fg(status_color));
    console_frame.render_widget(title, area_safe);
}
