#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::app_ui;

use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::style::Stylize;

use crate::the_types;
use ratatui::{prelude::*, widgets::*};

use std::str;

pub fn render_url(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_ui::DownApp,
    box_title: &str,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let draw_name = the_app.podcast_url.clone();
    let is_edit = the_app.ui_state == the_types::UiState::State001NewPodcastUrl;
    render_box(area_safe, console_frame, box_title, draw_name, is_edit);
}

pub fn render_name(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_ui::DownApp,
    box_title: &str,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let draw_name = the_app.podcast_name.clone();
    let is_edit = the_app.ui_state == the_types::UiState::State002NewPodcastName;
    render_box(area_safe, console_frame, box_title, draw_name, is_edit);
}
pub fn render_box(
    draw_area: Rect,
    console_frame: &mut Frame,
    the_title: &str,
    draw_name: String,
    is_edit: bool,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let has_chars = draw_name.len() > 0;
    let box_style: ratatui::style::Style = if is_edit || has_chars {
        Style::new().white().on_black()
    } else {
        Style::new().dark_gray().on_black() // https://docs.rs/crossterm/latest/crossterm/style/enum.Color.html
    };
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

pub fn render_add_podcast(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_ui::DownApp,
    box_title: &str,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let create_block = |title: &'static str| Block::new().gray().title(title.bold());
    let paragraph = Paragraph::new(box_title).white().block(create_block(""));
    if the_app.podcast_url.len() > 0 && the_app.podcast_name.len() > 0 {
        console_frame.render_widget(paragraph.on_green(), area_safe);
    } else {
        console_frame.render_widget(paragraph.on_dark_gray(), area_safe);
    }
}