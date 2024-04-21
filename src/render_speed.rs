#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::{app_state, const_globals};

use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::style::Stylize;

use ratatui::{prelude::*, widgets::*};

pub fn render_prefix(
    console_frame: &mut Frame,
    draw_area: Rect,
    _the_app: &mut app_state::DownApp,
    box_title: &str,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let title = Block::new().title(box_title.bold().white());
    console_frame.render_widget(title, area_safe);
}

pub fn render_resources(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_state::DownApp,
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
