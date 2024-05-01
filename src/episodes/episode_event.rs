#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::chunks::episode_threads;
use crate::consts::area_rects;
use crate::consts::areas_consts;
use crate::consts::const_globals;
use crate::misc::misc_fun;
use crate::state::app_state;

use crossterm::event::MouseEvent;
use ratatui::prelude::*;

use ratatui::layout::Rect;
use ratatui::widgets::*;

pub fn episode_hover(
    the_frame: &mut Frame,
    the_app: &mut app_state::DownApp,
    hover_event: MouseEvent,
) {
    let column = hover_event.column;
    let row = hover_event.row;
    let episode_area = area_rects::get_episode_area(the_frame);
    if area_rects::point_in_rect(column, row, episode_area) {
        the_app.hover_element = app_state::HOVER_EPISODES.to_string();
        the_app.episode_hover_row = (row - episode_area.y) as usize;
    }
}
pub fn get_episode_hover_index(episode_index: usize, the_app: &app_state::DownApp) -> i32 {
    let mut episode_hover_id: i32 = const_globals::NO_EPISODES_HOVER;
    let index_i32 = episode_index as i32;
    let episode_cur_top_row = the_app.scrolled_episodes_pos as i32;
    let singed_index = index_i32 + 1 - episode_cur_top_row;
    let singed_hover_row = the_app.episode_hover_row as i32;
    if singed_index == singed_hover_row {
        episode_hover_id = index_i32;
    }
    episode_hover_id
}
pub fn episode_click(the_app: &mut app_state::DownApp, episode_click: MouseEvent) -> () {
    let scroll_offest_epi = the_app.scrolled_episodes_pos;
    let num_episodes = the_app.ordered_episodes.len();
    let is_below_last = misc_fun::below_episodes(episode_click, scroll_offest_epi, num_episodes);
    if !is_below_last {
        let current_row = episode_click.row as usize;
        let m_ev_kind = episode_click.kind;
        if misc_fun::left_click(m_ev_kind) {
            let chunk_start_y_podcast: usize = areas_consts::START_Y_PODCAST as usize;
            let the_offset = scroll_offest_epi + current_row - chunk_start_y_podcast - 1;
            let the_choice = &the_app.ordered_episodes[the_offset];
            let sel_podcast = the_app.selected_podcast.to_string();
            let sel_episode = the_choice.to_string();
            let url_episode = the_app.episode_2_url[the_choice].to_string();
            if !the_app.local_episode_files.contains_key(&sel_episode) {
                episode_threads::queue_episode_download(
                    the_app,
                    sel_podcast,
                    sel_episode,
                    url_episode,
                );
            }
        }
    }
}

pub fn clicked_episodes(
    the_app: &mut app_state::DownApp,
    the_click: MouseEvent,
    the_frame: &mut Frame,
) -> () {
    let column = the_click.column;
    let row = the_click.row;
    let episode_area = area_rects::get_episode_area(the_frame);
    let click_on_title = row == episode_area.y;
    if area_rects::point_in_rect(column, row, episode_area) && !click_on_title {
        // NB - ignore click on title
        episode_click(the_app, the_click);
    }
}

pub fn episode_vscroll(
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
        &mut the_app.state_scroll_episodes,
    );
}
