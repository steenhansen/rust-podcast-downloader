#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::app_state;
use crate::area_rects;

use crossterm::event::{MouseEvent, MouseEventKind};
use ratatui::prelude::*;

pub fn do_pod_scroll(
    the_app: &mut app_state::DownApp,
    scroll_event: MouseEvent,
    console_frame: &mut Frame,
) -> () {
    let column = scroll_event.column;
    let row = scroll_event.row;
    let podcast_area = area_rects::get_podcast_area(console_frame);
    if area_rects::point_in_rect(column, row, podcast_area) {
        scroll_podcasts(the_app, scroll_event);
    }
    let elastic_epi_area = area_rects::get_episode_area(console_frame);
    if area_rects::point_in_rect(column, row, elastic_epi_area) {
        scroll_episodes(the_app, scroll_event);
    }
}

fn scroll_podcasts(the_app: &mut app_state::DownApp, scroll_event: MouseEvent) {
    if scroll_event.kind == MouseEventKind::ScrollUp {
        the_app.scrolled_podcasts_pos = the_app.scrolled_podcasts_pos.saturating_sub(1);
        the_app.state_scroll_podcasts = the_app
            .state_scroll_podcasts
            .position(the_app.scrolled_podcasts_pos);
    }
    if scroll_event.kind == MouseEventKind::ScrollDown {
        let num_podcasts = the_app.ordered_podcasts.len();
        let pod_scroll_pos = the_app.scrolled_podcasts_pos;
        if pod_scroll_pos + 1 < num_podcasts {
            the_app.scrolled_podcasts_pos = the_app.scrolled_podcasts_pos.saturating_add(1);
            the_app.state_scroll_podcasts = the_app
                .state_scroll_podcasts
                .position(the_app.scrolled_podcasts_pos);
        }
    }
}

fn scroll_episodes(the_app: &mut app_state::DownApp, scroll_event: MouseEvent) {
    if scroll_event.kind == MouseEventKind::ScrollUp {
        the_app.scrolled_episodes_pos = the_app.scrolled_episodes_pos.saturating_sub(1);
        the_app.state_scroll_episodes = the_app
            .state_scroll_episodes
            .position(the_app.scrolled_episodes_pos);
    }
    if scroll_event.kind == MouseEventKind::ScrollDown {
        let num_episodes = the_app.ordered_episodes.len();
        let epi_scroll_pos = the_app.scrolled_episodes_pos;
        if epi_scroll_pos + 1 < num_episodes {
            the_app.scrolled_episodes_pos = the_app.scrolled_episodes_pos.saturating_add(1);
            the_app.state_scroll_episodes = the_app
                .state_scroll_episodes
                .position(the_app.scrolled_episodes_pos);
        }
    }
}
