#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_globals;
use crate::state::app_state;
use ratatui::layout::Rect;
use ratatui::{prelude::*, widgets::*};

pub fn render_close(console_frame: &mut Frame, the_app: &app_state::DownApp, draw_area: Rect) {
    let area_safe = draw_area.intersection(console_frame.size());
    let mut close_text_color = const_globals::BTN_EVERY_TEXT_READY;
    let mut close_background_color = const_globals::BTN_EVERY_BACK_READY;
    if the_app.hover_element == app_state::HOVER_OK_DIALOG {
        close_text_color = const_globals::BTN_EVERY_TEXT_HOVER;
        close_background_color = const_globals::BTN_EVERY_BACK_HOVER;
    }

    let the_style = Style::default()
        .fg(close_text_color)
        .bg(close_background_color);

    let paragraph = Paragraph::new("\n  Ok")
        .block(Block::new().borders(Borders::NONE))
        .style(the_style);
    console_frame.render_widget(paragraph, area_safe);
}
