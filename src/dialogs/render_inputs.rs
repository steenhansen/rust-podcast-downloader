#[allow(unused)]
use log::{debug, info, trace, warn};

use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::style::Stylize;
use ratatui::{prelude::*, widgets::*};
use std::str;

pub fn render_box(
    draw_area: Rect,
    console_frame: &mut Frame,
    the_title: &str,
    draw_name: String,
    url_color: Color,
    is_edit: bool,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let box_style = Style::default().fg(url_color);
    let paragraph = Paragraph::new(draw_name.clone())
        .block(Block::new().title(the_title).borders(Borders::ALL))
        .style(box_style)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    console_frame.render_widget(paragraph, area_safe);

    if is_edit {
        let so_far_len = draw_name.len();
        let end_space = area_safe.x + so_far_len as u16;
        let white_square = Rect {
            x: end_space + 1,
            y: area_safe.y + 1,
            width: 1,
            height: 1,
        };
        let cursor = " ".bold().style(Style::new().black().on_white());
        console_frame.render_widget(cursor, white_square);
    }
}
