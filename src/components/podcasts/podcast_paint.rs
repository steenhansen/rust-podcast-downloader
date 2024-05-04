#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::components::podcasts::podcast_happenings;
use crate::consts::const_areas;
use crate::consts::const_colors;
use crate::state::state_app;

use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::{prelude::*, widgets::*};

pub const PODCAST_SELECTED: Color = Color::White;
pub const PODCAST_NOT_SELECTED: Color = Color::DarkGray;

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
            podcast_border_color = const_colors::PODCASTS_HOVER;
        } else {
            podcast_border_color = const_colors::PODCASTS_READY;
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
        .map(|(podcast_index, mut podcast_name)| {
            let happening_podcast_hover_id =
                podcast_happenings::happening_happening_clicked_podcasts(podcast_index, &the_app);
            let mut podcast_text_color = wait_color;
            let mut podcast_back_color = Color::Black;
            if wait_color == Color::Reset {
                if happening_podcast_hover_id >= 0 {
                    podcast_text_color = Color::White; //const_colors::A_PODCAST_TEXT_HOVER;
                    podcast_back_color = Color::Black; //const_colors::A_PODCAST_BACK_HOVER;
                    if the_app.podcast_file_types.contains_key(&podcast_name) {
                        let lower_extension = the_app
                            .podcast_file_types
                            .get(&podcast_name)
                            .cloned()
                            .unwrap_or("unknown-file-type".to_string());
                        let upper_extension = lower_extension.to_uppercase();
                        podcast_name = podcast_name.to_owned() + " ." + &upper_extension;
                    }
                } else if podcast_name == selected_podcast {
                    podcast_text_color = PODCAST_SELECTED;
                } else {
                    podcast_text_color = PODCAST_NOT_SELECTED;
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

    let mut pod_list_height = 0;
    if area_frame.height > 9 {
        pod_list_height = area_frame.height - 9;
    }

    let elastic_podcasts_area = Rect {
        x: const_areas::START_X_PODCAST,
        y: const_areas::START_Y_PODCAST,
        width: const_areas::WIDTH_PODCAST,
        height: pod_list_height,
    };
    elastic_podcasts_area
}
