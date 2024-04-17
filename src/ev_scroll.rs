#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::close_error;
//use crate::const_areas;
use crate::misc_fun;
use crate::render_app;
use crossterm::event::{MouseEvent, MouseEventKind};
use ratatui::prelude::*;

pub fn do_pod_scroll(
    the_app: &mut render_app::DownApp,
    mouse_event: MouseEvent,
    console_frame: &mut Frame,
) -> () {
    let column = mouse_event.column;
    let row = mouse_event.row;
    let elastic_pod_area = close_error::get_podcast_area(console_frame);
    if misc_fun::point_in_rect(column, row, elastic_pod_area) {
        if mouse_event.kind == MouseEventKind::ScrollUp {
            the_app.scrolled_podcasts = the_app.scrolled_podcasts.saturating_sub(1);
            the_app.state_scroll_podcasts = the_app
                .state_scroll_podcasts
                .position(the_app.scrolled_podcasts);
        }

        if mouse_event.kind == MouseEventKind::ScrollDown {
            let num_podcasts = the_app.ordered_podcasts.len();
            let pod_scroll_pos = the_app.scrolled_podcasts;
            if pod_scroll_pos + 1 < num_podcasts {
                the_app.scrolled_podcasts = the_app.scrolled_podcasts.saturating_add(1);
                the_app.state_scroll_podcasts = the_app
                    .state_scroll_podcasts
                    .position(the_app.scrolled_podcasts);
            }
        }
    }
    let elastic_epi_area = close_error::get_episode_area(console_frame);
    if misc_fun::point_in_rect(column, row, elastic_epi_area) {
        if mouse_event.kind == MouseEventKind::ScrollUp {
            the_app.scrolled_episodes = the_app.scrolled_episodes.saturating_sub(1);
            the_app.state_scroll_episodes = the_app
                .state_scroll_episodes
                .position(the_app.scrolled_episodes);
        }

        if mouse_event.kind == MouseEventKind::ScrollDown {
            let num_episodes = the_app.ordered_episodes.len();
            let epi_scroll_pos = the_app.scrolled_episodes;
            if epi_scroll_pos + 1 < num_episodes {
                the_app.scrolled_episodes = the_app.scrolled_episodes.saturating_add(1);
                the_app.state_scroll_episodes = the_app
                    .state_scroll_episodes
                    .position(the_app.scrolled_episodes);
            }
        }
    }
}
