#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::consts_areas;
use crate::consts::consts_globals;
use crate::globals::g_speed;
use crate::misc::misc_ui;
use crate::state::state_app;

use crossterm::event::MouseEvent;
use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::{prelude::*, widgets::*};

pub fn resources_show(
    console_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    app_dim: bool,
    is_downloading_paused: bool,
) {
    let mut wait_color = consts_globals::NORMAL_BORDER_COL;
    if is_downloading_paused {
        wait_color = consts_globals::PAUSE_COLOR;
    } else if app_dim {
        wait_color = consts_globals::DIMMED_BACKGROUND_WAIT;
    }
    resources_render(
        console_frame,
        consts_areas::RESOURCE_AREA,
        the_app,
        "Internet Resource Load",
        wait_color,
    );
}

pub fn resources_hover(the_app: &mut state_app::DownApp, hover_event: MouseEvent) {
    let column = hover_event.column;
    let row = hover_event.row;
    if misc_ui::rect_point_in(column, row, consts_areas::RESOURCE_AREA) {
        the_app.hover_element = state_app::HOVER_RESOURCES.to_string();
    }
}

pub fn resources_clicked(the_app: &mut state_app::DownApp, the_click: MouseEvent) -> () {
    let column = the_click.column;
    let row = the_click.row;
    if misc_ui::rect_point_in(column, row, consts_areas::RESOURCE_AREA) {
        let speed_chosen = row - consts_areas::RESOURCE_Y_START - 1;
        the_app.fast_med_slow = speed_chosen;
        g_speed::speed_change(speed_chosen);
    }
}

pub fn resources_render(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut state_app::DownApp,
    box_title: &str,
    wait_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());

    let resource_text_color = match wait_color {
        Color::Reset => {
            if the_app.hover_element == state_app::HOVER_RESOURCES {
                consts_globals::RESOURCE_HOVER
            } else {
                consts_globals::RESOURCE_READY
            }
        }
        _ => wait_color,
    };

    let box_style = Style::default().fg(resource_text_color);

    let fast_radio = match the_app.fast_med_slow == consts_globals::RESOURCE_FAST {
        true => "[X] ",
        false => "[O] ",
    };
    let med_radio = match the_app.fast_med_slow == consts_globals::RESOURCE_MED {
        true => "\n[X] ",
        false => "\n[O] ",
    };
    let slow_radio = match the_app.fast_med_slow == consts_globals::RESOURCE_SLOW {
        true => "\n[X] ",
        false => "\n[O] ",
    };

    let the_text = fast_radio.to_owned()
        + consts_globals::RADIO_RESOURCES[0]
        + med_radio
        + consts_globals::RADIO_RESOURCES[1]
        + slow_radio
        + consts_globals::RADIO_RESOURCES[2];

    let paragraph = Paragraph::new(the_text)
        .block(Block::new().title(box_title).borders(Borders::ALL))
        .style(box_style)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    console_frame.render_widget(paragraph, area_safe);
}
