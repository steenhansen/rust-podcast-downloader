#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::area_rects;
use crate::state::app_state;

use crate::consts::const_globals;

use crate::podcasts::podcast_files;
use ratatui::layout::Rect;
use ratatui::{prelude::*, widgets::*};

pub fn show_feed(
    console_frame: &mut Frame,
    the_app: &mut app_state::DownApp,
    dim_background: bool,
    is_downloading_paused: bool,
) {
    let podcast_name = the_app.selected_podcast.clone();
    if podcast_name != "" {
        let rss_feed = podcast_files::get_filename(podcast_name);

        let address_area = area_rects::get_feed_area(console_frame, &rss_feed);
        let mut wait_color = const_globals::NORMAL_BORDER_COL;
        if is_downloading_paused {
            wait_color = const_globals::PAUSE_COLOR;
        } else if dim_background {
            wait_color = const_globals::DIMMED_BACKGROUND_WAIT;
        }
        render_feed(console_frame, address_area, the_app, &rss_feed, wait_color);
    }
}

pub fn render_feed(
    console_frame: &mut Frame,
    draw_area: Rect,
    _the_app: &mut app_state::DownApp,
    box_title: &str,
    status_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let title = Block::new()
        .title(box_title)
        .style(Style::default().fg(status_color));
    console_frame.render_widget(title, area_safe);
}
