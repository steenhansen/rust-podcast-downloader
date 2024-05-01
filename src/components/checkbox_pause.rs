#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::area_rects;
use crate::consts::areas_consts;
use crate::consts::const_globals;
use crate::consts::the_types;
use crate::globals::g_current_active;
use crate::globals::g_pause_io;
use crate::state::app_state;

use crossterm::event::MouseEvent;
use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::{prelude::*, widgets::*};

pub fn show_pause(console_frame: &mut Frame, the_app: &mut app_state::DownApp) {
    if the_app.selected_podcast != "" {
        let is_currently_paused = g_pause_io::is_cur_paused();
        let pause_area = areas_consts::PAUSE_AREA;
        if is_currently_paused {
            render_pause(console_frame, pause_area, the_app, "[✓] Downloads Paused");
        } else {
            render_pause(console_frame, pause_area, the_app, "[ ] Pause Downloads");
        }
    }
}

pub fn pause_hover(the_app: &mut app_state::DownApp, hover_event: MouseEvent) {
    let column = hover_event.column;
    let row = hover_event.row;
    if area_rects::point_in_rect(column, row, areas_consts::PAUSE_AREA) {
        the_app.hover_element = app_state::HOVER_PAUSE.to_string();
    }
}

pub fn active_btn_pause(the_app: &mut app_state::DownApp) -> bool {
    let pause_active = g_current_active::active_downloading() > 0
        && (the_app.ui_state == the_types::UiState::State103ShowEpisodes
            || the_app.ui_state == the_types::UiState::State203DownloadingEvery
            || the_app.ui_state == the_types::UiState::State401DownloadPaused);
    pause_active
}

pub fn check_pause(the_app: &mut app_state::DownApp, the_click: MouseEvent) -> () {
    if active_btn_pause(the_app) {
        let column = the_click.column;
        let row = the_click.row;
        let pause_area = areas_consts::PAUSE_AREA;
        if area_rects::point_in_rect(column, row, pause_area) {
            let pause_state = g_pause_io::pause_flip();
            if pause_state {
                the_app.pause_type_103_203 = the_app.ui_state;
                the_app.ui_state = the_types::UiState::State401DownloadPaused;
            } else {
                the_app.ui_state = the_app.pause_type_103_203;
            }
        }
    }
}

pub fn render_pause(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_state::DownApp,
    box_title: &str,
) {
    let area_safe = draw_area.intersection(console_frame.size());

    let mut the_style = Style::default().fg(const_globals::PAUSE_DEAD);
    if g_current_active::active_downloading() > 0 {
        if the_app.hover_element == app_state::HOVER_PAUSE {
            the_style = Style::default().fg(const_globals::PAUSE_HOVER);
        } else {
            the_style = Style::default().fg(const_globals::PAUSE_READY);
        }
    }
    let title = Block::new().title(box_title).style(the_style);
    console_frame.render_widget(title, area_safe);
}
