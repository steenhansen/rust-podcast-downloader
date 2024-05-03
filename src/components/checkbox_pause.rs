#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::consts_areas;
use crate::consts::consts_globals;
use crate::consts::consts_types;
use crate::globals::g_active;
use crate::globals::g_pause;
use crate::misc::misc_ui;
use crate::state::state_app;

use crossterm::event::MouseEvent;
use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::{prelude::*, widgets::*};

pub fn pause_show(console_frame: &mut Frame, the_app: &mut state_app::DownApp) {
    if the_app.selected_podcast != "" {
        let is_currently_paused = g_pause::pause_currently();
        let pause_area = consts_areas::PAUSE_AREA;
        if is_currently_paused {
            pause_render(console_frame, pause_area, the_app, "[✓] Downloads Paused");
        } else {
            pause_render(console_frame, pause_area, the_app, "[ ] Pause Downloads");
        }
    }
}

pub fn pause_hover(the_app: &mut state_app::DownApp, hover_event: MouseEvent) {
    let column = hover_event.column;
    let row = hover_event.row;
    if misc_ui::rect_point_in(column, row, consts_areas::PAUSE_AREA) {
        the_app.hover_element = state_app::HOVER_PAUSE.to_string();
    }
}

pub fn pause_active(the_app: &mut state_app::DownApp) -> bool {
    let pause_active = g_active::active_downloading() > 0
        && (the_app.ui_state == consts_types::UiState::State103ShowEpisodes
            || the_app.ui_state == consts_types::UiState::State203DownloadingEvery
            || the_app.ui_state == consts_types::UiState::State401DownloadPaused);
    pause_active
}

pub fn pause_clicked(the_app: &mut state_app::DownApp, the_click: MouseEvent) -> () {
    if pause_active(the_app) {
        let column = the_click.column;
        let row = the_click.row;
        let pause_area = consts_areas::PAUSE_AREA;
        if misc_ui::rect_point_in(column, row, pause_area) {
            let pause_state = g_pause::pause_flip();
            if pause_state {
                the_app.pause_type_103_203 = the_app.ui_state;
                the_app.ui_state = consts_types::UiState::State401DownloadPaused;
            } else {
                the_app.ui_state = the_app.pause_type_103_203;
            }
        }
    }
}

pub fn pause_render(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut state_app::DownApp,
    box_title: &str,
) {
    let area_safe = draw_area.intersection(console_frame.size());

    let mut the_style = Style::default().fg(consts_globals::PAUSE_DEAD);
    if g_active::active_downloading() > 0 {
        if the_app.hover_element == state_app::HOVER_PAUSE {
            the_style = Style::default().fg(consts_globals::PAUSE_HOVER);
        } else {
            the_style = Style::default().fg(consts_globals::PAUSE_READY);
        }
    }
    let title = Block::new().title(box_title).style(the_style);
    console_frame.render_widget(title, area_safe);
}
