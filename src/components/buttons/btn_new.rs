#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::components::input_address;
use crate::components::input_name;
use crate::components::podcasts::podcast_types;
use crate::consts::consts_areas;
use crate::consts::consts_globals;
use crate::consts::consts_rects;
use crate::consts::consts_types;
use crate::state::state_app;

use crossterm::event::MouseEvent;
use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::{prelude::*, widgets::*};
use std::str;

pub fn new_show(
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

    new_podcast_render(
        console_frame,
        consts_areas::NEW_PODCAST_AREA,
        the_app,
        "\n Add New Podcast",
        wait_color,
    );
}

pub fn new_hover(the_app: &mut state_app::DownApp, hover_event: MouseEvent) {
    let column = hover_event.column;
    let row = hover_event.row;
    if consts_rects::rect_point_in(column, row, consts_areas::NEW_PODCAST_AREA) {
        the_app.hover_element = state_app::HOVER_BTN_NEW.to_string();
    }
}

pub fn new_clicked(the_app: &mut state_app::DownApp, the_click: MouseEvent) {
    let column = the_click.column;
    let row = the_click.row;
    input_name::name_clicked(the_app, the_click);
    input_address::address_clicked(the_app, the_click);
    if consts_rects::rect_point_in(column, row, consts_areas::NEW_PODCAST_AREA) {
        if new_btn_active(the_app) {
            podcast_types::types_create_pod_dir(the_app).expect("create-dir-err");
            the_app.ui_state = consts_types::UiState::State003ClickedNew;
        }
    }
}

pub fn new_btn_active(the_app: &mut state_app::DownApp) -> bool {
    let new_active = the_app.new_podcast_url.len() > 0 && the_app.new_podcast_name.len() > 0;
    new_active
}

pub fn new_podcast_render(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut state_app::DownApp,
    box_title: &str,
    wait_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());

    let mut new_text_color = consts_globals::BTN_EVERY_TEXT_DEAD;
    let mut new_background_color = wait_color;
    if wait_color == Color::Reset {
        if new_btn_active(the_app) {
            if the_app.hover_element == state_app::HOVER_BTN_NEW {
                new_text_color = consts_globals::BTN_EVERY_TEXT_HOVER;
                new_background_color = consts_globals::BTN_EVERY_BACK_HOVER;
            } else {
                new_text_color = consts_globals::BTN_EVERY_TEXT_READY;
                new_background_color = consts_globals::BTN_EVERY_BACK_READY;
            }
        } else {
            new_text_color = consts_globals::BTN_EVERY_TEXT_DEAD;
            new_background_color = consts_globals::BTN_EVERY_BACK_DEAD;
        }
    }

    let text_style = Style::default().fg(new_text_color);
    let background_style = Style::default().bg(new_background_color);

    let paragraph = Paragraph::new(box_title)
        .style(background_style)
        .block(Block::new().style(text_style));

    console_frame.render_widget(paragraph, area_safe);
}