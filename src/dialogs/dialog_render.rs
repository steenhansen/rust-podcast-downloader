#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_colors;

use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::style::Stylize;
use ratatui::{prelude::*, widgets::*};
use std::str;

pub fn render_box(
    draw_area: Rect,
    console_frame: &mut Frame,
    the_title: &str,
    the_example: &str,
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

    let area_safe = draw_area.intersection(console_frame.size());
    let span2 = Span::styled(the_example, Style::new().fg(Color::DarkGray)).to_right_aligned_line();

    console_frame.render_widget(span2, area_safe);

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

pub fn dialog_centered(dialog_size: Rect, frame_area: Rect) -> Rect {
    let middle_x = frame_area.width / 2 - dialog_size.width / 2;
    let middle_y = frame_area.height / 2 - dialog_size.height / 2;

    let centered_area: ratatui::layout::Rect = Rect {
        x: middle_x,
        y: middle_y,
        width: dialog_size.width,
        height: dialog_size.height,
    };
    centered_area
}

pub fn ok_centered(ok_size: Rect, centered_area: Rect) -> Rect {
    let ok_x = centered_area.x + centered_area.width / 2 - ok_size.width / 2;
    let ok_y = centered_area.y + ok_size.y;

    let ok_area: ratatui::layout::Rect = Rect {
        x: ok_x,
        y: ok_y,
        width: ok_size.width,
        height: ok_size.height,
    };
    ok_area
}

pub fn dialog_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Gray))
        .title(Span::styled(title, Style::default()))
}

pub fn dialog_yes_no(hover_element: String, hover_type: String, the_label: &str) -> Paragraph {
    let mut yes_text_color = const_colors::BTN_EVERY_TEXT_READY;
    let mut yes_background_color = const_colors::BTN_EVERY_BACK_READY;
    if hover_element == hover_type {
        yes_text_color = const_colors::BTN_EVERY_TEXT_HOVER;
        yes_background_color = const_colors::BTN_EVERY_BACK_HOVER;
    }
    let yes_style = Style::default().fg(yes_text_color).bg(yes_background_color);

    let paragraph = Paragraph::new(the_label)
        .block(Block::new().borders(Borders::NONE))
        .style(yes_style);
    paragraph
}
