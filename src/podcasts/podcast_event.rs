#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::podcasts::podcast_show;

use crate::consts::area_rects;
use crate::consts::areas_consts;
use crate::consts::const_globals;
use crate::consts::the_types;
use crate::globals::g_current_active;
use crate::misc::misc_fun;
use crate::state::app_state;

use crossterm::event::MouseEvent;
use ratatui::layout::Rect;
use ratatui::{prelude::*, widgets::*};

pub fn podcast_hover(
    the_frame: &mut Frame,
    the_app: &mut app_state::DownApp,
    hover_event: MouseEvent,
) {
    let column = hover_event.column;
    let row = hover_event.row;
    let podcast_area = podcast_show::get_podcast_area(the_frame);
    if area_rects::point_in_rect(column, row, podcast_area) {
        the_app.hover_element = app_state::HOVER_PODCASTS.to_string();
        the_app.podcast_hover_row = (row - podcast_area.y) as usize;
    }
}

pub fn podcasts_vscroll(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_state::DownApp,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    console_frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        area_safe,
        &mut the_app.state_scroll_podcasts,
    );
}

pub fn get_podcast_hover_index(episode_index: usize, the_app: &app_state::DownApp) -> i32 {
    let mut podcast_hover_id: i32 = const_globals::NO_PODCASTS_HOVER;
    let index_i32 = episode_index as i32;
    let podcast_cur_top_row = the_app.scrolled_podcasts_pos as i32;
    let singed_index = index_i32 + 1 - podcast_cur_top_row;
    let singed_hover_row = the_app.podcast_hover_row as i32;
    if singed_index == singed_hover_row {
        podcast_hover_id = index_i32;
    }
    podcast_hover_id
}

pub fn check_podcasts(
    the_app: &mut app_state::DownApp,
    the_click: MouseEvent,
    the_frame: &mut Frame,
) -> () {
    let column = the_click.column;
    let row = the_click.row;
    let podcast_area = podcast_show::get_podcast_area(the_frame);
    let click_on_title = row == podcast_area.y;
    if area_rects::point_in_rect(column, row, podcast_area) && !click_on_title {
        // NB - ignore click on title
        podcast_click(the_app, the_click);
    }
}

pub fn podcast_click(the_app: &mut app_state::DownApp, mouse_event: MouseEvent) -> () {
    let scroll_offest_pod = the_app.scrolled_podcasts_pos;
    let num_podcasts = the_app.ordered_podcasts.len();
    let is_below_last = misc_fun::below_podcasts(mouse_event, scroll_offest_pod, num_podcasts);
    if !is_below_last {
        let row = mouse_event.row as usize;
        let m_ev_kind = mouse_event.kind;
        if misc_fun::left_click(m_ev_kind) {
            let chunk_start_y_podcast: usize = areas_consts::START_Y_PODCAST as usize;
            let the_offset = scroll_offest_pod + row - chunk_start_y_podcast - 1;
            let the_choice = &the_app.ordered_podcasts[the_offset];

            the_app.selected_podcast = the_choice.to_string();
            the_app.ui_state = the_types::UiState::State101ReadingRss;
            // qbert - possible wrongness
            g_current_active::new_pod_clear(); // clear all others ??? when switch?
            return;
        }
    }
    //    the_app.selected_podcast = "-podcast-click-outside-".to_string();
    the_app.selected_podcast = const_globals::BLANK_PODCAST.to_string();
}
