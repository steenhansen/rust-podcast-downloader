//             //https://docs.rs/ratatui/latest/src/tabs/tabs.rs.html#144
#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::{const_globals, render_app};

use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::style::Stylize;

use crate::the_types;
use ratatui::{prelude::*, widgets::*};

use crossterm::cursor::MoveTo;
use crossterm::event::KeyCode;
use crossterm::execute;
use std::io::stdout;

use std::str;

//   Style::new().dark_gray().on_black()
pub fn render_status(
    console_frame: &mut Frame,
    draw_area: Rect,
    _the_app: &mut render_app::DownApp,
    box_title: &str,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let title = Block::new()
        .title_alignment(Alignment::Center)
        .title(box_title.bold().dark_gray());
    console_frame.render_widget(title, area_safe);
}

pub fn render_close(console_frame: &mut Frame, draw_area: Rect, box_title: &str) {
    let area_safe = draw_area.intersection(console_frame.size());
    let paragraph = Paragraph::new(box_title)
        .block(Block::new().borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    console_frame.render_widget(paragraph, area_safe);
}

pub fn render_button(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut render_app::DownApp,
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

pub fn render_name(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut render_app::DownApp,
    box_title: &str,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let draw_name = the_app.podcast_name.clone();
    let is_edit = the_app.ui_state == the_types::UiState::State002NewPodcastName;
    render_box(area_safe, console_frame, box_title, draw_name, is_edit);
}

pub fn render_title(
    console_frame: &mut Frame,
    draw_area: Rect,
    _the_app: &mut render_app::DownApp,
    box_title: &str,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let title = Block::new()
        .title_alignment(Alignment::Center)
        .title(box_title.bold());
    console_frame.render_widget(title, area_safe);
}

pub fn edit_text(ed_text: &String, the_code: KeyCode) -> String {
    let mut ed_text2 = ed_text.clone();
    match the_code {
        KeyCode::Char(c) => {
            ed_text2.push(c);
        }
        KeyCode::Backspace => {
            ed_text2.pop();
        }
        _ => {}
    }
    execute!(
        stdout(),
        MoveTo(13, 1),
        crossterm::cursor::SetCursorStyle::BlinkingUnderScore
    )
    .unwrap();
    ed_text2
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

pub fn render_url(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut render_app::DownApp,
    box_title: &str,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let draw_name = the_app.podcast_url.clone();
    let is_edit = the_app.ui_state == the_types::UiState::State001NewPodcastUrl;
    render_box(area_safe, console_frame, box_title, draw_name, is_edit);
}

pub fn render_radio(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut render_app::DownApp,
    box_title: &str,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let box_style = Style::new().white().on_black();

    let fast_radio = match the_app.fast_med_slow == 0 {
        true => "[X] ",
        false => "[O] ",
    };
    let med_radio = match the_app.fast_med_slow == 1 {
        true => "\n[X] ",
        false => "\n[O] ",
    };
    let slow_radio = match the_app.fast_med_slow == 2 {
        true => "\n[X] ",
        false => "\n[O] ",
    };

    let the_text = fast_radio.to_owned()
        + const_globals::RADIO_RESOURCES[0]
        + med_radio
        + const_globals::RADIO_RESOURCES[1]
        + slow_radio
        + const_globals::RADIO_RESOURCES[2];

    let paragraph = Paragraph::new(the_text)
        .block(Block::new().title(box_title).borders(Borders::ALL))
        .style(box_style)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    console_frame.render_widget(paragraph, area_safe);
}
