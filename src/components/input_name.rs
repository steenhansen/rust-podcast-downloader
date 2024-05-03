#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::consts_areas;
use crate::consts::consts_globals;
use crate::consts::consts_rects;
use crate::consts::consts_types;
use crate::dialogs::dialog_render;
use crate::state::state_app;

use crossterm::event::MouseEvent;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use std::str;

pub fn name_show(
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

    name_render(
        console_frame,
        consts_areas::NEW_NAME_AREA,
        the_app,
        "New Podcast Name─Nasa-Images",
        wait_color,
    );
}

pub fn name_hover(the_app: &mut state_app::DownApp, hover_event: MouseEvent) {
    let column = hover_event.column;
    let row = hover_event.row;
    if consts_rects::rect_point_in(column, row, consts_areas::NEW_NAME_AREA) {
        the_app.hover_element = state_app::HOVER_NEW_NAME.to_string();
    }
}

pub fn name_clicked(the_app: &mut state_app::DownApp, the_click: MouseEvent) {
    let column = the_click.column;
    let row = the_click.row;
    if consts_rects::rect_point_in(column, row, consts_areas::NEW_NAME_AREA) {
        the_app.ui_state = consts_types::UiState::State002NewPodcastName;
    }
}
pub fn name_render(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut state_app::DownApp,
    box_title: &str,
    wait_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let draw_name = the_app.new_podcast_name.clone();
    let is_edit = the_app.ui_state == consts_types::UiState::State002NewPodcastName;
    let has_chars = draw_name.len() > 0;
    let mut url_color = wait_color;

    if wait_color == Color::Reset {
        if is_edit || has_chars || the_app.hover_element == state_app::HOVER_NEW_NAME {
            url_color = consts_globals::INPUT_TEXT_HOVER
        } else {
            url_color = consts_globals::INPUT_TEXT_READY
        }
    }

    dialog_render::render_box(
        area_safe,
        console_frame,
        box_title,
        draw_name,
        url_color,
        is_edit,
    );
}
