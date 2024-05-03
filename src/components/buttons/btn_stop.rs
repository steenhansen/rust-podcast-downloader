#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::consts_areas;
use crate::consts::consts_globals;
use crate::consts::consts_types;
use crate::globals::g_active;
use crate::globals::g_stop;
use crate::misc::misc_ui;
use crate::state::state_app;

use crossterm::event::MouseEvent;
use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::{prelude::*, widgets::*};
use std::str;

pub fn stop_show(console_frame: &mut Frame, the_app: &mut state_app::DownApp) {
    stop_render(
        console_frame,
        consts_areas::STOP_PODCAST_AREA,
        the_app,
        "\n Stop Downloading",
    );
}

pub fn stop_hover(the_app: &mut state_app::DownApp, hover_event: MouseEvent) {
    let column = hover_event.column;
    let row = hover_event.row;
    if misc_ui::rect_point_in(column, row, consts_areas::STOP_PODCAST_AREA) {
        the_app.hover_element = state_app::HOVER_BTN_STOP.to_string();
    }
}

pub fn stop_active(the_app: &mut state_app::DownApp) -> bool {
    let stop_active = g_active::active_downloading() > 0
        && (the_app.ui_state == consts_types::UiState::State103ShowEpisodes
            || the_app.ui_state == consts_types::UiState::State203DownloadingEvery);
    stop_active
}

pub fn stop_clicked(the_app: &mut state_app::DownApp, the_click: MouseEvent) -> () {
    if stop_active(the_app) {
        let column = the_click.column;
        let row = the_click.row;
        let pause_area = consts_areas::STOP_PODCAST_AREA;
        if misc_ui::rect_point_in(column, row, pause_area) {
            the_app.download_deque.clear();
            g_stop::stop_true();
            the_app.ui_state = consts_types::UiState::State601KillingDownloads;
        }
    }
}

pub fn stop_render(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut state_app::DownApp,
    box_title: &str,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let mut stop_text_color = consts_globals::BTN_EVERY_TEXT_DEAD;
    let mut stop_background_color = consts_globals::BTN_EVERY_BACK_DEAD;

    if the_app.ui_state == consts_types::UiState::State401DownloadPaused {
        stop_text_color = consts_globals::PAUSE_TEXT_COLOR;
        stop_background_color = consts_globals::PAUSE_BACK_COLOR;
    } else if stop_active(the_app) {
        if the_app.hover_element == state_app::HOVER_BTN_STOP {
            stop_text_color = consts_globals::BTN_EVERY_TEXT_HOVER;
            stop_background_color = consts_globals::BTN_EVERY_BACK_HOVER;
        } else {
            stop_text_color = consts_globals::BTN_EVERY_TEXT_READY;
            stop_background_color = consts_globals::BTN_EVERY_BACK_READY;
        }
    }
    let text_style = Style::default().fg(stop_text_color);
    let background_style = Style::default().bg(stop_background_color);
    let paragraph = Paragraph::new(box_title)
        .style(background_style)
        .block(Block::new().style(text_style));

    console_frame.render_widget(paragraph, area_safe);
}
