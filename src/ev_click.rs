#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::app_state;
use crate::area_rects;
use crate::areas_consts;
use crate::episode_threads;
use crate::g_current_active;
use crate::g_pause_io;
use crate::g_resource_speed;
use crate::misc_fun;
use crate::podcast_scroll;
use crate::the_types;

use crossterm::event::{MouseButton::Left, MouseEvent, MouseEventKind};
use ratatui::prelude::*;

pub fn do_click_mouse(
    the_frame: &mut Frame,
    the_app: &mut app_state::DownApp,
    mouse_event: MouseEvent,
) -> bool {
    if the_app.ui_state != the_types::UiState::State101ReadingRss
        && the_app.ui_state != the_types::UiState::State102ShowWaiting
        && the_app.ui_state != the_types::UiState::State201AllEpisodes
        && the_app.ui_state != the_types::UiState::State202SureAllEpisodes
        && the_app.ui_state != the_types::UiState::State301WaitForPopErrorClose
        && the_app.ui_state != the_types::UiState::State401DownloadPaused
    {
        check_add(the_app, mouse_event);
        check_all(the_app, mouse_event);
        check_podcasts(the_app, mouse_event, the_frame);
        check_episodes(the_app, mouse_event, the_frame);
        check_resources(the_app, mouse_event);
    }
    check_pause(the_app, mouse_event);
    check_error_ok(the_app, mouse_event, the_frame);
    check_all_ok(the_app, mouse_event, the_frame);

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

fn check_add(the_app: &mut app_state::DownApp, the_click: MouseEvent) {
    let column = the_click.column;
    let row = the_click.row;
    if area_rects::point_in_rect(column, row, areas_consts::NEW_NAME_AREA) {
        the_app.ui_state = the_types::UiState::State002NewPodcastName;
    }
    if area_rects::point_in_rect(column, row, areas_consts::NEW_URL_AREA) {
        the_app.ui_state = the_types::UiState::State001NewPodcastUrl;
    }

    if area_rects::point_in_rect(column, row, areas_consts::ADD_PODCAST_AREA) {
        if the_app.new_podcast_url.len() > 0 && the_app.new_podcast_name.len() > 0 {
            podcast_scroll::create_pod_dir(the_app).expect("create-dir-err");
            the_app.ui_state = the_types::UiState::State003ClickedAdd;
        }
    }
}

fn check_all(the_app: &mut app_state::DownApp, the_click: MouseEvent) {
    let column = the_click.column;
    let row = the_click.row;
    if area_rects::point_in_rect(column, row, areas_consts::ALL_PODCAST_AREA) {
        if the_app.selected_podcast.len() > 0 {
            the_app.ui_state = the_types::UiState::State201AllEpisodes;
        }
    }
}

fn check_all_ok(
    the_app: &mut app_state::DownApp,
    the_click: MouseEvent,
    the_frame: &mut Frame,
) -> () {
    if the_app.ui_state == the_types::UiState::State201AllEpisodes {
        let column = the_click.column;
        let row = the_click.row;
        let error_close_area = area_rects::get_error_close_area(the_frame);
        if area_rects::point_in_rect(column, row, error_close_area) {
            the_app.ui_state = the_types::UiState::State202SureAllEpisodes;
        }
    }
}
fn check_error_ok(
    the_app: &mut app_state::DownApp,
    the_click: MouseEvent,
    the_frame: &mut Frame,
) -> () {
    if the_app.ui_state == the_types::UiState::State301WaitForPopErrorClose {
        let column = the_click.column;
        let row = the_click.row;
        let error_close_area = area_rects::get_error_close_area(the_frame);
        if area_rects::point_in_rect(column, row, error_close_area) {
            the_app.ui_state = the_types::UiState::StateNoFocus;
        }
    }
}

fn check_resources(the_app: &mut app_state::DownApp, the_click: MouseEvent) -> () {
    let column = the_click.column;
    let row = the_click.row;
    if area_rects::point_in_rect(column, row, areas_consts::RADIO_AREA) {
        let speed_chosen = row - areas_consts::RADIO_Y_START - 1;
        the_app.fast_med_slow = speed_chosen;
        g_resource_speed::change_speed(speed_chosen);
    }
}

fn check_podcasts(
    the_app: &mut app_state::DownApp,
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
    the_app: &mut app_state::DownApp,
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

fn check_pause(the_app: &mut app_state::DownApp, the_click: MouseEvent) -> () {
    let column = the_click.column;
    let row = the_click.row;
    let pause_area = areas_consts::PAUSE_AREA;
    if area_rects::point_in_rect(column, row, pause_area) {
        let pause_state = g_pause_io::pause_flip();
        if pause_state {
            the_app.ui_state = the_types::UiState::State401DownloadPaused;
        } else {
            the_app.ui_state = the_types::UiState::State402NotPaused;
        }
    }
}

fn podcast_click(the_app: &mut app_state::DownApp, mouse_event: MouseEvent) -> () {
    let scroll_offest_pod = the_app.scrolled_podcasts_pos;
    let num_podcasts = the_app.ordered_podcasts.len();
    let is_below_last = misc_fun::below_podcasts(mouse_event, scroll_offest_pod, num_podcasts);
    if !is_below_last {
        let row = mouse_event.row as usize;
        let m_ev_kind = mouse_event.kind;
        if left_click(m_ev_kind) {
            let chunk_start_y_podcast: usize = areas_consts::START_Y_PODCAST as usize;
            let the_offset = scroll_offest_pod + row - chunk_start_y_podcast - 1;
            let the_choice = &the_app.ordered_podcasts[the_offset];

            the_app.selected_podcast = the_choice.to_string();
            the_app.ui_state = the_types::UiState::State101ReadingRss;
            g_current_active::new_pod_clear();
            return;
        }
    }
    the_app.selected_podcast = "-podcast-click-outside-".to_string();
}

fn episode_click(the_app: &mut app_state::DownApp, episode_click: MouseEvent) -> () {
    let scroll_offest_epi = the_app.scrolled_episodes_pos;
    let num_episodes = the_app.ordered_episodes.len();
    let is_below_last = misc_fun::below_episodes(episode_click, scroll_offest_epi, num_episodes);
    if !is_below_last {
        let current_row = episode_click.row as usize;
        let m_ev_kind = episode_click.kind;
        if left_click(m_ev_kind) {
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

fn left_click(kind_click: MouseEventKind) -> bool {
    if kind_click == MouseEventKind::Down(Left) || kind_click == MouseEventKind::Up(Left) {
        return true;
    }
    false
}
