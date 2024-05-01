#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::area_rects;
use crate::consts::areas_consts;
use crate::consts::const_globals;
use crate::consts::the_types;
use crate::dialogs::render_inputs;
use crate::state::app_state;

use crossterm::event::MouseEvent;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use std::str;

pub fn show_address(
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

    render_url(
        console_frame,
        areas_consts::NEW_URL_AREA,
        the_app,
        "New Podcast URL─https://www.nasa.gov/feeds/iotd-feed",
        wait_color,
    );
}

// use the word address to stop url overpopulation

pub fn address_hover(the_app: &mut app_state::DownApp, hover_event: MouseEvent) {
    let column = hover_event.column;
    let row = hover_event.row;
    if area_rects::point_in_rect(column, row, areas_consts::NEW_URL_AREA) {
        the_app.hover_element = app_state::HOVER_NEW_URL.to_string();
    }
}

pub fn clicked_url(the_app: &mut app_state::DownApp, the_click: MouseEvent) {
    let column = the_click.column;
    let row = the_click.row;
    if area_rects::point_in_rect(column, row, areas_consts::NEW_URL_AREA) {
        the_app.ui_state = the_types::UiState::State001NewPodcastUrl;
    }
}

pub fn render_url(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_state::DownApp,
    box_title: &str,
    wait_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let draw_name = the_app.new_podcast_url.clone();
    let is_edit = the_app.ui_state == the_types::UiState::State001NewPodcastUrl;
    let has_chars = draw_name.len() > 0;
    let mut url_color = wait_color;

    if wait_color == Color::Reset {
        if is_edit || has_chars || the_app.hover_element == app_state::HOVER_NEW_URL {
            url_color = const_globals::INPUT_TEXT_HOVER
        } else {
            url_color = const_globals::INPUT_TEXT_READY
        }
    }

    render_inputs::render_box(
        area_safe,
        console_frame,
        box_title,
        draw_name,
        url_color,
        is_edit,
    );
}
