#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::components::podcasts::podcast_happenings;
use crate::consts::consts_areas;
use crate::consts::consts_globals;
use crate::state::state_app;

use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::{prelude::*, widgets::*};

pub fn paint_pod_list(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut state_app::DownApp,
    box_title: String,
    wait_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());

    let ordered_podcasts = the_app.ordered_podcasts.clone();
    let selected_podcast = the_app.selected_podcast.clone();

    let colored_pod_rows: Vec<Line> =
        paint_colored_podcasts(ordered_podcasts, selected_podcast, the_app, wait_color);
    the_app.state_scroll_podcasts = the_app
        .state_scroll_podcasts
        .content_length(colored_pod_rows.len());

    let mut podcast_border_color = wait_color;
    if wait_color == Color::Reset {
        if the_app.hover_element == state_app::HOVER_PODCASTS {
            podcast_border_color = consts_globals::PODCASTS_HOVER;
        } else {
            podcast_border_color = consts_globals::PODCASTS_READY;
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

pub fn paint_colored_podcasts(
    ordered_podcastsed: Vec<String>,
    selected_podcast: String,
    the_app: &mut state_app::DownApp,
    wait_color: Color,
) -> Vec<Line<'static>> {
    let text: Vec<Line> = ordered_podcastsed
        .into_iter()
        .enumerate()
        .map(|(podcast_index, podcast_name)| {
            let happening_podcast_hover_id =
                podcast_happenings::happening_happening_clicked_podcasts(podcast_index, &the_app);
            let mut podcast_text_color = wait_color;
            let mut podcast_back_color = Color::Black;
            if wait_color == Color::Reset {
                if happening_podcast_hover_id >= 0 {
                    podcast_text_color = consts_globals::A_PODCAST_TEXT_HOVER;
                    podcast_back_color = consts_globals::A_PODCAST_BACK_HOVER;
                } else if podcast_name == selected_podcast {
                    podcast_text_color = consts_globals::PODCAST_SELECTED;
                } else {
                    podcast_text_color = consts_globals::PODCAST_NOT_SELECTED;
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

pub fn paint_podcast_area(console_frame: &mut Frame) -> Rect {
    let area_frame = console_frame.size();
    let elastic_podcasts_area = Rect {
        x: consts_areas::START_X_PODCAST,
        y: consts_areas::START_Y_PODCAST,
        width: consts_areas::WIDTH_PODCAST,
        height: area_frame.height - 9,
    };
    elastic_podcasts_area
}
