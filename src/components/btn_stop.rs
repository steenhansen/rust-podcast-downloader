#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_globals;
use crate::consts::the_types;
use crate::globals::g_current_active;
use crate::globals::g_stop_io;
use crate::state::app_state;

use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::{prelude::*, widgets::*};
use std::str;

use crate::consts::area_rects;
use crate::consts::areas_consts;

use crossterm::event::MouseEvent;

pub fn show_stop(console_frame: &mut Frame, the_app: &mut app_state::DownApp) {
    render_stop(
        console_frame,
        areas_consts::STOP_PODCAST_AREA,
        the_app,
        "\n Stop Downloading",
    );
}

pub fn stop_hover(the_app: &mut app_state::DownApp, hover_event: MouseEvent) {
    let column = hover_event.column;
    let row = hover_event.row;
    if area_rects::point_in_rect(column, row, areas_consts::STOP_PODCAST_AREA) {
        the_app.hover_element = app_state::HOVER_BTN_STOP.to_string();
    }
}

pub fn active_btn_stop(the_app: &mut app_state::DownApp) -> bool {
    let stop_active = g_current_active::active_downloading() > 0
        && (the_app.ui_state == the_types::UiState::State103ShowEpisodes
            || the_app.ui_state == the_types::UiState::State203DownloadingEvery);
    stop_active
}

pub fn check_stop(the_app: &mut app_state::DownApp, the_click: MouseEvent) -> () {
    if active_btn_stop(the_app) {
        let column = the_click.column;
        let row = the_click.row;
        let pause_area = areas_consts::STOP_PODCAST_AREA;
        if area_rects::point_in_rect(column, row, pause_area) {
            the_app.download_deque.clear();
            g_stop_io::start_stoping();
            the_app.ui_state = the_types::UiState::State601KillingDownloads;
        }
    }
}

pub fn render_stop(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_state::DownApp,
    box_title: &str,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let mut stop_text_color = const_globals::BTN_EVERY_TEXT_DEAD;
    let mut stop_background_color = const_globals::BTN_EVERY_BACK_DEAD;

    if the_app.ui_state == the_types::UiState::State401DownloadPaused {
        stop_text_color = const_globals::PAUSE_TEXT_COLOR;
        stop_background_color = const_globals::PAUSE_BACK_COLOR;
    } else if active_btn_stop(the_app) {
        if the_app.hover_element == app_state::HOVER_BTN_STOP {
            stop_text_color = const_globals::BTN_EVERY_TEXT_HOVER;
            stop_background_color = const_globals::BTN_EVERY_BACK_HOVER;
        } else {
            stop_text_color = const_globals::BTN_EVERY_TEXT_READY;
            stop_background_color = const_globals::BTN_EVERY_BACK_READY;
        }
    }
    let text_style = Style::default().fg(stop_text_color);
    let background_style = Style::default().bg(stop_background_color);
    let paragraph = Paragraph::new(box_title)
        .style(background_style)
        .block(Block::new().style(text_style));

    console_frame.render_widget(paragraph, area_safe);
}
