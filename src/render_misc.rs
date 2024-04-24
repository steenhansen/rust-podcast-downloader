#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::app_state;

use ratatui::layout::Rect;
use ratatui::{prelude::*, widgets::*};

pub fn render_close(console_frame: &mut Frame, draw_area: Rect, box_title: &str) {
    let area_safe = draw_area.intersection(console_frame.size());
    let paragraph = Paragraph::new(box_title)
        .block(Block::new().borders(Borders::ALL))
        .style(Style::default().add_modifier(Modifier::BOLD));
    console_frame.render_widget(paragraph, area_safe);
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

pub fn render_title(
    console_frame: &mut Frame,
    draw_area: Rect,
    _the_app: &mut app_state::DownApp,
    box_title: &str,
    title_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let title = Block::new()
        .title_alignment(Alignment::Center)
        .title(box_title)
        .style(
            Style::default()
                .fg(title_color)
                // .bg(const_globals::TITLE_BACKGROUND)
                .add_modifier(Modifier::BOLD | Modifier::ITALIC | Modifier::UNDERLINED),
        );
    console_frame.render_widget(title, area_safe);
}
