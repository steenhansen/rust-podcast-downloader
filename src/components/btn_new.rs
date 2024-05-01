#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::podcasts::podcast_state;

use crate::components::input_address;
use crate::components::input_name;
use crate::consts::area_rects;
use crate::consts::areas_consts;
use crate::state::app_state;

use crate::consts::const_globals;
use crate::consts::the_types;

use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::{prelude::*, widgets::*};
use std::str;

use crossterm::event::MouseEvent;

pub fn show_new(
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

    render_new_podcast(
        console_frame,
        areas_consts::NEW_PODCAST_AREA,
        the_app,
        "\n Add New Podcast",
        wait_color,
    );
}

pub fn new_hover(the_app: &mut app_state::DownApp, hover_event: MouseEvent) {
    let column = hover_event.column;
    let row = hover_event.row;
    if area_rects::point_in_rect(column, row, areas_consts::NEW_PODCAST_AREA) {
        the_app.hover_element = app_state::HOVER_BTN_NEW.to_string();
    }
}

pub fn check_new(the_app: &mut app_state::DownApp, the_click: MouseEvent) {
    let column = the_click.column;
    let row = the_click.row;
    input_name::clicked_name(the_app, the_click);
    input_address::clicked_url(the_app, the_click);
    if area_rects::point_in_rect(column, row, areas_consts::NEW_PODCAST_AREA) {
        if active_btn_new(the_app) {
            podcast_state::create_pod_dir(the_app).expect("create-dir-err");
            the_app.ui_state = the_types::UiState::State003ClickedNew;
        }
    }
}

pub fn active_btn_new(the_app: &mut app_state::DownApp) -> bool {
    let new_active = the_app.new_podcast_url.len() > 0 && the_app.new_podcast_name.len() > 0;
    new_active
}

pub fn render_new_podcast(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_state::DownApp,
    box_title: &str,
    wait_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());

    let mut new_text_color = const_globals::BTN_EVERY_TEXT_DEAD;
    let mut new_background_color = wait_color;
    if wait_color == Color::Reset {
        if active_btn_new(the_app) {
            if the_app.hover_element == app_state::HOVER_BTN_NEW {
                new_text_color = const_globals::BTN_EVERY_TEXT_HOVER;
                new_background_color = const_globals::BTN_EVERY_BACK_HOVER;
            } else {
                new_text_color = const_globals::BTN_EVERY_TEXT_READY;
                new_background_color = const_globals::BTN_EVERY_BACK_READY;
            }
        } else {
            new_text_color = const_globals::BTN_EVERY_TEXT_DEAD;
            new_background_color = const_globals::BTN_EVERY_BACK_DEAD;
        }
    }

    let text_style = Style::default().fg(new_text_color);
    let background_style = Style::default().bg(new_background_color);

    let paragraph = Paragraph::new(box_title)
        .style(background_style)
        .block(Block::new().style(text_style));

    console_frame.render_widget(paragraph, area_safe);
}
