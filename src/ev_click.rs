#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::close_error;
use crate::const_areas;
use crate::down_app;

use crate::get_episode;
use crate::misc_fun;
use crate::podcast_scroll;
use crate::the_types;
use crossterm::event::{MouseButton::Left, MouseEvent, MouseEventKind};
use ratatui::prelude::*;

pub fn do_click_mouse(
    the_frame: &mut Frame,
    the_app: &mut down_app::DownApp,
    mouse_event: MouseEvent,
) -> bool {
    let column = mouse_event.column;
    let row = mouse_event.row;

    if misc_fun::point_in_rect(column, row, const_areas::NEW_PODCAST_URL) {
        the_app.ui_state = the_types::UiState::State001NewPodcastUrl;
    }
    if misc_fun::point_in_rect(column, row, const_areas::NEW_PODCAST_NAME) {
        the_app.ui_state = the_types::UiState::State002NewPodcastName;
    }

    if misc_fun::point_in_rect(column, row, const_areas::ADD_AREA) {
        if the_app.podcast_url.len() > 0 && the_app.podcast_name.len() > 0 {
            podcast_scroll::create_pod_dir(the_app).unwrap();
            the_app.ui_state = the_types::UiState::State003ClickedAdd;
        }
    }
    if misc_fun::point_in_rect(column, row, const_areas::PODCAST_AREA) {
        do_pod_click(the_app, mouse_event);
    }
    if misc_fun::point_in_rect(column, row, const_areas::EPISODE_AREA) {
        do_episode_click(the_app, mouse_event);
    }
    let pop_close_area = close_error::get_pop_close_area(the_frame); // changes with frame
    if misc_fun::point_in_rect(column, row, pop_close_area) {
        the_app.ui_state = the_types::UiState::StateNoFocus;
    }

    let up_right_area = close_error::get_end_prog_area(the_frame);
    if misc_fun::point_in_rect(column, row, up_right_area) {
        do_pod_click(the_app, mouse_event);
        return true;
    }
    false
}

fn do_pod_click(the_app: &mut down_app::DownApp, mouse_event: MouseEvent) -> () {
    let scroll_offest_pod = the_app.scrolled_podcasts;
    let num_podcasts = the_app.ordered_podcasts.len();
    let is_o = misc_fun::below_podcasts(mouse_event, scroll_offest_pod, num_podcasts);
    if !is_o {
        let row = mouse_event.row as usize;
        let m_ev_kind = mouse_event.kind;
        if left_click(m_ev_kind) {
            let the_offset = scroll_offest_pod + row - const_areas::START_Y_PODCAST_US - 1;
            let the_choice = &the_app.ordered_podcasts[the_offset];

            the_app.selected_podcast = the_choice.to_string();
            the_app.ui_state = the_types::UiState::State101ReadingRss;
            return;
        }
    }
    the_app.selected_podcast = "-podcast-click-outside-".to_string();
    return;
}

fn left_click(kind_click: MouseEventKind) -> bool {
    if kind_click == MouseEventKind::Down(Left) || kind_click == MouseEventKind::Up(Left) {
        return true;
    }
    false
}

fn do_episode_click(the_app: &mut down_app::DownApp, episode_click: MouseEvent) -> () {
    let scroll_offest_epi = the_app.scrolled_episodes;
    let num_episodes = the_app.ordered_episodes.len();
    let is_o = misc_fun::below_episodes(episode_click, scroll_offest_epi, num_episodes);
    if !is_o {
        let row = episode_click.row as usize;
        let m_ev_kind = episode_click.kind;
        if left_click(m_ev_kind) {
            let the_offset = scroll_offest_epi + row - const_areas::START_Y_PODCAST_US - 1;
            let the_choice = &the_app.ordered_episodes[the_offset];

            the_app.selected_episode = the_choice.to_string();

            let sel_podcast = the_app.selected_podcast.to_string();
            let sel_episode = the_choice.to_string();
            let url_episode = the_app.episode_2_url[the_choice].to_string();
            get_episode::spawn_getter(sel_podcast, sel_episode, url_episode);
            return;
        }
    }
    the_app.selected_episode = "-episode-click-outside-".to_string();
    return;
}
