#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::components::podcasts::podcast_directory;
use crate::consts::consts_globals;
use crate::consts::consts_rects;
use crate::state::state_app;

use ratatui::layout::Rect;
use ratatui::{prelude::*, widgets::*};

pub fn feed_show(
    console_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    app_dim: bool,
    is_downloading_paused: bool,
) {
    let podcast_name = the_app.selected_podcast.clone();
    if podcast_name != "" {
        let rss_feed = podcast_directory::directory_get_filename(podcast_name);

        let address_area = consts_rects::rect_feed(console_frame, &rss_feed);
        let mut wait_color = consts_globals::NORMAL_BORDER_COL;
        if is_downloading_paused {
            wait_color = consts_globals::PAUSE_COLOR;
        } else if app_dim {
            wait_color = consts_globals::DIMMED_BACKGROUND_WAIT;
        }
        feed_render(console_frame, address_area, the_app, &rss_feed, wait_color);
    }
}

pub fn feed_render(
    console_frame: &mut Frame,
    draw_area: Rect,
    _the_app: &mut state_app::DownApp,
    box_title: &str,
    status_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let title = Block::new()
        .title(box_title)
        .style(Style::default().fg(status_color));
    console_frame.render_widget(title, area_safe);
}
