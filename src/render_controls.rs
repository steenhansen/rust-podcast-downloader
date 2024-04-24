#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::{app_state, const_globals};

use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::{prelude::*, widgets::*};

pub fn render_pause(
    console_frame: &mut Frame,
    draw_area: Rect,
    _the_app: &mut app_state::DownApp,
    box_title: &str,
    dim_background: bool,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let resource_text_color;
    if dim_background {
        resource_text_color = const_globals::DIMMED_BACKGROUND_WAIT;
    } else {
        resource_text_color = const_globals::NORMAL_TEXT_COL;
    }
    let box_style = Style::default().fg(resource_text_color);
    let title = Block::new().title(box_title).style(box_style);
    console_frame.render_widget(title, area_safe);
}

pub fn render_resources(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_state::DownApp,
    box_title: &str,
    wait_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());

    let resource_text_color = match wait_color {
        Color::Reset => const_globals::NORMAL_TEXT_COL,
        _ => wait_color,
    };

    let box_style = Style::default().fg(resource_text_color);

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
