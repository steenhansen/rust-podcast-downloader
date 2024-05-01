#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::area_rects;
use crate::consts::areas_consts;
use crate::consts::const_globals;
use crate::globals::g_resource_speed;
use crate::state::app_state;

use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::{prelude::*, widgets::*};

use crossterm::event::MouseEvent;

pub fn show_resources(
    console_frame: &mut Frame,
    the_app: &mut app_state::DownApp,
    dim_background: bool,
    is_downloading_paused: bool,
) {
    let mut wait_color = const_globals::NORMAL_BORDER_COL;
    if is_downloading_paused {
        wait_color = const_globals::PAUSE_COLOR;
    } else if dim_background {
        wait_color = const_globals::DIMMED_BACKGROUND_WAIT;
    }
    render_resources(
        console_frame,
        areas_consts::RESOURCE_AREA,
        the_app,
        "Internet Resource Load",
        wait_color,
    );
}

pub fn resources_hover(the_app: &mut app_state::DownApp, hover_event: MouseEvent) {
    let column = hover_event.column;
    let row = hover_event.row;
    if area_rects::point_in_rect(column, row, areas_consts::RESOURCE_AREA) {
        the_app.hover_element = app_state::HOVER_RESOURCES.to_string();
    }
}

pub fn check_resources(the_app: &mut app_state::DownApp, the_click: MouseEvent) -> () {
    let column = the_click.column;
    let row = the_click.row;
    if area_rects::point_in_rect(column, row, areas_consts::RESOURCE_AREA) {
        let speed_chosen = row - areas_consts::RESOURCE_Y_START - 1;
        the_app.fast_med_slow = speed_chosen;
        g_resource_speed::change_speed(speed_chosen);
    }
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
        Color::Reset => {
            if the_app.hover_element == app_state::HOVER_RESOURCES {
                const_globals::RESOURCE_HOVER
            } else {
                const_globals::RESOURCE_READY
            }
        }
        _ => wait_color,
    };

    let box_style = Style::default().fg(resource_text_color);

    let fast_radio = match the_app.fast_med_slow == const_globals::RESOURCE_FAST {
        true => "[X] ",
        false => "[O] ",
    };
    let med_radio = match the_app.fast_med_slow == const_globals::RESOURCE_MED {
        true => "\n[X] ",
        false => "\n[O] ",
    };
    let slow_radio = match the_app.fast_med_slow == const_globals::RESOURCE_SLOW {
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
