#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::consts_areas;
use crate::consts::consts_globals;
use crate::media::media_threads;
use crate::misc::misc_ui;
use crate::state::state_app;

use crossterm::event::MouseEvent;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn rect_episode(console_frame: &mut Frame) -> Rect {
    let area_frame = console_frame.size();
    let elastic_episodes_area = Rect {
        x: consts_areas::START_X_EPISODE,
        y: consts_areas::START_Y_EPISODE,
        width: area_frame.width - 10,
        height: area_frame.height - 9,
    };
    elastic_episodes_area
}

pub fn acts_episode_hover(
    the_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    hover_event: MouseEvent,
) {
    let column = hover_event.column;
    let row = hover_event.row;
    let episode_area = rect_episode(the_frame);
    if misc_ui::rect_point_in(column, row, episode_area) {
        the_app.hover_element = state_app::HOVER_EPISODES.to_string();
        the_app.acts_episode_hover_row = (row - episode_area.y) as usize;
    }
}

pub fn acts_episode_hover_index(episode_index: usize, the_app: &state_app::DownApp) -> i32 {
    let mut acts_episode_hover_id: i32 = consts_globals::NO_EPISODES_HOVER;
    let index_i32 = episode_index as i32;
    let episode_cur_top_row = the_app.scrolled_episodes_pos as i32;
    let singed_index = index_i32 + 1 - episode_cur_top_row;
    let singed_hover_row = the_app.acts_episode_hover_row as i32;
    if singed_index == singed_hover_row {
        acts_episode_hover_id = index_i32;
    }
    acts_episode_hover_id
}

pub fn acts_episode_click(the_app: &mut state_app::DownApp, acts_episode_click: MouseEvent) -> () {
    let scroll_offest_epi = the_app.scrolled_episodes_pos;
    let num_episodes = the_app.ordered_episodes.len();
    let is_below_last =
        misc_ui::below_episodes(acts_episode_click, scroll_offest_epi, num_episodes);
    if !is_below_last {
        let current_row = acts_episode_click.row as usize;
        let m_ev_kind = acts_episode_click.kind;
        if misc_ui::left_click(m_ev_kind) {
            let chunk_start_y_podcast: usize = consts_areas::START_Y_PODCAST as usize;
            let the_offset = scroll_offest_epi + current_row - chunk_start_y_podcast - 1;
            let the_choice = &the_app.ordered_episodes[the_offset];
            let sel_podcast = the_app.selected_podcast.to_string();
            let sel_episode = the_choice.to_string();
            let url_episode = the_app.episode_2_url[the_choice].to_string();
            if !the_app.local_episode_files.contains_key(&sel_episode) {
                media_threads::threads_queue(the_app, sel_podcast, sel_episode, url_episode);
            }
        }
    }
}

pub fn acts_clicked_episodes(
    the_app: &mut state_app::DownApp,
    the_click: MouseEvent,
    the_frame: &mut Frame,
) -> () {
    let column = the_click.column;
    let row = the_click.row;
    let episode_area = rect_episode(the_frame);
    let click_on_title = row == episode_area.y;
    if misc_ui::rect_point_in(column, row, episode_area) && !click_on_title {
        // NB - ignore click on title
        acts_episode_click(the_app, the_click);
    }
}

pub fn acts_episode_vscroll(
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
        &mut the_app.state_scroll_episodes,
    );
}
