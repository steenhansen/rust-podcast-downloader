#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::app_ui;
use crate::area_rects;
use crate::areas_consts;

use crate::episode_threads;
use crate::g_resource_speed;
use crate::misc_fun;
use crate::podcast_files;
use crate::podcast_scroll;
use crate::the_types;
use crossterm::event::{MouseButton::Left, MouseEvent, MouseEventKind};
use ratatui::prelude::*;

pub fn do_click_mouse(
    the_frame: &mut Frame,
    the_app: &mut app_ui::DownApp,
    mouse_event: MouseEvent,
) -> bool {
    check_add(the_app, mouse_event);
    check_podcasts(the_app, mouse_event, the_frame);
    check_episodes(the_app, mouse_event, the_frame);
    check_resources(the_app, mouse_event);
    check_prefix(the_app, mouse_event, the_frame);
    check_popup(the_app, mouse_event, the_frame);
    let is_quit = check_quit(mouse_event, the_frame);
    is_quit
}

fn check_quit(the_click: MouseEvent, the_frame: &mut Frame) -> bool {
    let column = the_click.column;
    let row = the_click.row;
    let quit_area = area_rects::get_quit_area(the_frame);
    if area_rects::point_in_rect(column, row, quit_area) {
        return true;
    }
    false
}

fn check_add(the_app: &mut app_ui::DownApp, the_click: MouseEvent) {
    let column = the_click.column;
    let row = the_click.row;
    if area_rects::point_in_rect(column, row, areas_consts::NEW_NAME_AREA) {
        the_app.ui_state = the_types::UiState::State002NewPodcastName;
    }
    if area_rects::point_in_rect(column, row, areas_consts::NEW_URL_AREA) {
        the_app.ui_state = the_types::UiState::State001NewPodcastUrl;
    }

    if area_rects::point_in_rect(column, row, areas_consts::ADD_PODCAST_AREA) {
        if the_app.podcast_url.len() > 0 && the_app.podcast_name.len() > 0 {
            podcast_scroll::create_pod_dir(the_app).unwrap();
            the_app.ui_state = the_types::UiState::State003ClickedAdd;
        }
    }
}

fn check_resources(the_app: &mut app_ui::DownApp, the_click: MouseEvent) -> () {
    let column = the_click.column;
    let row = the_click.row;
    if area_rects::point_in_rect(column, row, areas_consts::RADIO_AREA) {
        let speed_chosen = row - areas_consts::RADIO_Y_START - 1;
        the_app.fast_med_slow = speed_chosen;
        g_resource_speed::change_speed(speed_chosen);
    }
}

fn check_podcasts(
    the_app: &mut app_ui::DownApp,
    the_click: MouseEvent,
    the_frame: &mut Frame,
) -> () {
    let column = the_click.column;
    let row = the_click.row;
    let podcast_area = area_rects::get_podcast_area(the_frame);
    if area_rects::point_in_rect(column, row, podcast_area) {
        podcast_click(the_app, the_click);
    }
}

fn check_episodes(
    the_app: &mut app_ui::DownApp,
    the_click: MouseEvent,
    the_frame: &mut Frame,
) -> () {
    let column = the_click.column;
    let row = the_click.row;
    let episode_area = area_rects::get_episode_area(the_frame);
    if area_rects::point_in_rect(column, row, episode_area) {
        episode_click(the_app, the_click);
    }
}

fn check_prefix(the_app: &mut app_ui::DownApp, the_click: MouseEvent, the_frame: &mut Frame) -> () {
    let column = the_click.column;
    let row = the_click.row;
    let prefix_area = area_rects::get_prefix_area(the_frame);
    if area_rects::point_in_rect(column, row, prefix_area) {
        podcast_scroll::flip_prefix(the_app).unwrap();
        podcast_files::get_epi_list(the_app).unwrap();
    }
}

fn check_popup(the_app: &mut app_ui::DownApp, the_click: MouseEvent, the_frame: &mut Frame) -> () {
    let column = the_click.column;
    let row = the_click.row;
    let pop_close_area = area_rects::get_pop_close_area(the_frame);
    if area_rects::point_in_rect(column, row, pop_close_area) {
        the_app.ui_state = the_types::UiState::StateNoFocus;
    }
}

fn podcast_click(the_app: &mut app_ui::DownApp, mouse_event: MouseEvent) -> () {
    let scroll_offest_pod = the_app.scrolled_podcasts;
    let num_podcasts = the_app.ordered_podcasts.len();
    let is_o = misc_fun::below_podcasts(mouse_event, scroll_offest_pod, num_podcasts);
    if !is_o {
        let row = mouse_event.row as usize;
        let m_ev_kind = mouse_event.kind;
        if left_click(m_ev_kind) {
            let start_y_podcast: usize = areas_consts::START_Y_PODCAST as usize;
            let the_offset = scroll_offest_pod + row - start_y_podcast - 1;
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

fn episode_click(the_app: &mut app_ui::DownApp, episode_click: MouseEvent) -> () {
    let scroll_offest_epi = the_app.scrolled_episodes;
    let num_episodes = the_app.ordered_episodes.len();
    let is_o = misc_fun::below_episodes(episode_click, scroll_offest_epi, num_episodes);
    if !is_o {
        let row = episode_click.row as usize;
        let m_ev_kind = episode_click.kind;
        if left_click(m_ev_kind) {
            let start_y_podcast: usize = areas_consts::START_Y_PODCAST as usize;
            let the_offset = scroll_offest_epi + row - start_y_podcast - 1;
            let the_choice = &the_app.ordered_episodes[the_offset];

            the_app.selected_episode = the_choice.to_string();

            let sel_podcast = the_app.selected_podcast.to_string();
            let sel_episode = the_choice.to_string();
            let url_episode = the_app.episode_2_url[the_choice].to_string();
            episode_threads::spawn_getter(sel_podcast, sel_episode, url_episode);
            return;
        }
    }
    the_app.selected_episode = "-episode-click-outside-".to_string();
    return;
}
