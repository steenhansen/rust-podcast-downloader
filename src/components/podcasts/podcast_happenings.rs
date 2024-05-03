#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::components::podcasts::podcast_paint;
use crate::consts::consts_areas;
use crate::consts::consts_globals;
use crate::consts::consts_types;
use crate::globals::g_active;
use crate::misc::misc_ui;
use crate::state::state_app;

use crossterm::event::MouseEvent;
use ratatui::layout::Rect;
use ratatui::{prelude::*, widgets::*};

pub fn happening_podcast_hover(
    the_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    hover_event: MouseEvent,
) {
    let column = hover_event.column;
    let row = hover_event.row;
    let podcast_area = podcast_paint::paint_podcast_area(the_frame);
    if misc_ui::rect_point_in(column, row, podcast_area) {
        the_app.hover_element = state_app::HOVER_PODCASTS.to_string();
        the_app.happening_podcast_hover_row = (row - podcast_area.y) as usize;
    }
}

pub fn happening_podcasts_vscroll(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut state_app::DownApp,
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

pub fn happening_happening_clicked_podcasts(
    episode_index: usize,
    the_app: &state_app::DownApp,
) -> i32 {
    let mut happening_podcast_hover_id: i32 = consts_globals::NO_PODCASTS_HOVER;
    let index_i32 = episode_index as i32;
    let podcast_cur_top_row = the_app.scrolled_podcasts_pos as i32;
    let singed_index = index_i32 + 1 - podcast_cur_top_row;
    let singed_hover_row = the_app.happening_podcast_hover_row as i32;
    if singed_index == singed_hover_row {
        happening_podcast_hover_id = index_i32;
    }
    happening_podcast_hover_id
}

pub fn happening_clicked_podcasts(
    the_app: &mut state_app::DownApp,
    the_click: MouseEvent,
    the_frame: &mut Frame,
) -> () {
    let column = the_click.column;
    let row = the_click.row;
    let podcast_area = podcast_paint::paint_podcast_area(the_frame);
    let click_on_title = row == podcast_area.y;
    if misc_ui::rect_point_in(column, row, podcast_area) && !click_on_title {
        // NB - ignore click on title
        happening_podcast_click(the_app, the_click);
    }
}

pub fn happening_podcast_click(the_app: &mut state_app::DownApp, mouse_event: MouseEvent) -> () {
    let scroll_offest_pod = the_app.scrolled_podcasts_pos;
    let num_podcasts = the_app.ordered_podcasts.len();
    let is_below_last = misc_ui::below_podcasts(mouse_event, scroll_offest_pod, num_podcasts);
    if !is_below_last {
        let row = mouse_event.row as usize;
        let m_ev_kind = mouse_event.kind;
        if misc_ui::left_click(m_ev_kind) {
            let chunk_start_y_podcast: usize = consts_areas::START_Y_PODCAST as usize;
            let the_offset = scroll_offest_pod + row - chunk_start_y_podcast - 1;
            let the_choice = &the_app.ordered_podcasts[the_offset];

            the_app.selected_podcast = the_choice.to_string();
            the_app.ui_state = consts_types::UiState::State101ReadingRss;
            // qbert - possible wrongness
            g_active::active_clear(); // clear all others ??? when switch?
            return;
        }
    }
    the_app.selected_podcast = consts_globals::BLANK_PODCAST.to_string();
}
