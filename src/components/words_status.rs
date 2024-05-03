#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::consts_globals;
use crate::consts::consts_rects;
use crate::globals::g_active;
use crate::state::state_app;

use ratatui::layout::Rect;
use ratatui::{prelude::*, widgets::*};

pub fn status_show(
    console_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    app_dim: bool,
    is_downloading_paused: bool,
) {
    let status_area = consts_rects::rect_status(console_frame);
    let mut wait_color = consts_globals::NORMAL_BORDER_COL;
    if is_downloading_paused {
        wait_color = consts_globals::PAUSE_COLOR;
    } else if app_dim {
        wait_color = consts_globals::DIMMED_BACKGROUND_WAIT;
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
