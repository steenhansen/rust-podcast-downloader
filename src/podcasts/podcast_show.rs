#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::podcasts::podcast_event;

use crate::consts::areas_consts;
use crate::consts::const_globals;
use crate::state::app_state;

use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::{prelude::*, widgets::*};

pub fn render_pod_list(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_state::DownApp,
    box_title: String,
    wait_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());

    let ordered_podcasts = the_app.ordered_podcasts.clone();
    let selected_podcast = the_app.selected_podcast.clone();
    // qbert
    let colored_pod_rows: Vec<Line> =
        colored_podcasts(ordered_podcasts, selected_podcast, the_app, wait_color);
    the_app.state_scroll_podcasts = the_app
        .state_scroll_podcasts
        .content_length(colored_pod_rows.len());

    let mut podcast_border_color = wait_color;
    if wait_color == Color::Reset {
        if the_app.hover_element == app_state::HOVER_PODCASTS {
            podcast_border_color = const_globals::PODCASTS_HOVER;
        } else {
            podcast_border_color = const_globals::PODCASTS_READY;
        }
    }

    let create_block = |title: String| {
        Block::bordered()
            .title(title.bold())
            .style(Style::default().fg(podcast_border_color))
    };
    let paragraph = Paragraph::new(colored_pod_rows.clone())
        .block(create_block(box_title))
        .scroll((the_app.scrolled_podcasts_pos as u16, 0));
    console_frame.render_widget(paragraph, area_safe);
}

// qbert
pub fn colored_podcasts(
    ordered_podcastsed: Vec<String>,
    selected_podcast: String,
    the_app: &mut app_state::DownApp,
    wait_color: Color,
) -> Vec<Line<'static>> {
    let text: Vec<Line> = ordered_podcastsed
        .into_iter()
        .enumerate()
        .map(|(podcast_index, podcast_name)| {
            let podcast_hover_id = podcast_event::get_podcast_hover_index(podcast_index, &the_app);
            let mut podcast_text_color = wait_color;
            let mut podcast_back_color = Color::Black;
            if wait_color == Color::Reset {
                if podcast_hover_id >= 0 {
                    podcast_text_color = const_globals::A_PODCAST_TEXT_HOVER;
                    podcast_back_color = const_globals::A_PODCAST_BACK_HOVER;
                } else if podcast_name == selected_podcast {
                    podcast_text_color = const_globals::PODCAST_SELECTED;
                } else {
                    podcast_text_color = const_globals::PODCAST_NOT_SELECTED;
                }
            }
            let the_style = Style::default()
                .fg(podcast_text_color)
                .bg(podcast_back_color);
            Line::styled(podcast_name, the_style)
        })
        .collect();

    text
}

pub fn get_podcast_area(console_frame: &mut Frame) -> Rect {
    let area_frame = console_frame.size();
    let elastic_podcasts_area = Rect {
        x: areas_consts::START_X_PODCAST,
        y: areas_consts::START_Y_PODCAST,
        width: areas_consts::WIDTH_PODCAST,
        height: area_frame.height - 9,
    };
    elastic_podcasts_area
}
