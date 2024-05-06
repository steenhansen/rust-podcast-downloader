#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::components::podcasts::podcast_directory;
use crate::consts::const_areas;
use crate::consts::const_colors;
use crate::state::state_app;

use ratatui::layout::Rect;
use ratatui::{prelude::*, widgets::*};

pub fn rect_feed(console_frame: &mut Frame, the_url: &str) -> Rect {
    let area_frame = console_frame.size();
    let url_width = the_url.len() as u16;
    let mut left_start: i16 = area_frame.width as i16 - url_width as i16;
    if left_start < const_areas::MIN_FEED_X_START {
        left_start = const_areas::MIN_FEED_X_START;
    }
    let up_right_area = Rect {
        x: left_start as u16,
        y: area_frame.height - 1,
        width: url_width,
        height: 1,
    };
    up_right_area
}

pub fn feed_show(
    console_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    app_dim: bool,
    is_downloading_paused: bool,
) {
    let podcast_name = the_app.selected_podcast.clone();
    if podcast_name != "" {
        let rss_feed = podcast_directory::directory_get_filename(podcast_name);

        let address_area = rect_feed(console_frame, &rss_feed);
        let mut wait_color = const_colors::NORMAL_BORDER_COL;
        if is_downloading_paused {
            wait_color = const_colors::PAUSE_COLOR;
        } else if app_dim {
            wait_color = const_colors::DIMMED_BACKGROUND_WAIT;
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
