#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::app_state;
use crate::const_globals;
use crate::episode_colors;
use crate::g_current_active;

use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::{prelude::*, widgets::*};
use std::collections::HashMap;

pub fn episode_vscroll(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_state::DownApp,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    console_frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        area_safe,
        &mut the_app.state_scroll_episodes,
    );
}

pub fn render_epi_list(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_state::DownApp,
    box_title: String,
    wait_color: Color,
) {
    let ordered_episodes = the_app.ordered_episodes.clone();
    let local_episode_files = the_app.local_episode_files.clone();
    let colored_epi_rows: Vec<Line> =
        colored_episodes(ordered_episodes, local_episode_files, the_app, wait_color);
    let episode_title = format!("{}——{}", the_app.selected_podcast, box_title);
    let area_safe = draw_area.intersection(console_frame.size());
    the_app.state_scroll_podcasts = the_app
        .state_scroll_podcasts
        .content_length(colored_epi_rows.len());

    let episode_border_color = match wait_color {
        Color::Reset => const_globals::NORMAL_BORDER_COL,
        _ => wait_color,
    };

    let create_block = |title: String| {
        Block::bordered()
            .title(title)
            .style(Style::default().fg(episode_border_color))
    };
    let paragraph = Paragraph::new(colored_epi_rows.clone())
        .green()
        .block(create_block(episode_title))
        .scroll((the_app.scrolled_episodes_pos as u16, 0));
    console_frame.render_widget(paragraph, area_safe);
}

pub fn render_epi_list_empty(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_state::DownApp,
    _box_title: String,
    wait_color: Color,
) {
    let episode_title = format!("{}", the_app.selected_podcast);
    let episode_text_color = match wait_color {
        Color::Reset => const_globals::LOADING_EPISODES_COL,
        _ => wait_color,
    };

    let loading_files = Span::styled(
        "loading...".to_owned(),
        Style::default().fg(episode_text_color),
    );
    let colored_epi_rows: Vec<Line> = [Line::from(loading_files)].to_vec();

    let area_safe = draw_area.intersection(console_frame.size());
    let episode_border_color = match wait_color {
        Color::Reset => const_globals::NORMAL_BORDER_COL,
        _ => wait_color,
    };
    the_app.state_scroll_podcasts = the_app
        .state_scroll_podcasts
        .content_length(colored_epi_rows.len());
    let create_block = |title: String| {
        Block::bordered()
            .title(title)
            .style(Style::default().fg(episode_border_color))
    };
    let paragraph = Paragraph::new(colored_epi_rows.clone())
        .green()
        .block(create_block(episode_title))
        .scroll((the_app.scrolled_episodes_pos as u16, 0));
    console_frame.render_widget(paragraph, area_safe);
}

fn colored_episodes(
    ordered_episodes: Vec<String>,
    local_episode_files: HashMap<String, String>,
    the_app: &mut app_state::DownApp,
    wait_color: Color,
) -> Vec<Line<'static>> {
    let mut the_waiting: HashMap<String, String> = HashMap::new();
    let selected_podcast = &the_app.selected_podcast;
    for (_sel_podcast, media_fname, _url_episode) in &the_app.download_deque {
        let full_epi_name = format!("{selected_podcast}/{media_fname}");
        the_waiting.insert(full_epi_name.to_string(), full_epi_name.to_string());
    }
    let text: Vec<Line> = ordered_episodes
        .into_iter()
        .map(|episode_name| {
            if local_episode_files.contains_key(&episode_name) {
                let old_local_file = episode_colors::color_old_epi(&episode_name, wait_color);
                return Line::from(old_local_file);
            } else {
                let cur_read_status = g_current_active::G_CURRENT_ACTIVE
                    .lock()
                    .expect("chunk-memory-err");
                let full_epi_name = format!("{selected_podcast}/{episode_name}");
                match cur_read_status.get(&full_epi_name) {
                    Some(num_bytes) => {
                        return episode_colors::start_or_downloading_or_done(
                            &num_bytes,
                            &episode_name,
                            wait_color,
                        );
                    }
                    None => {
                        return episode_colors::possible_or_waiting(
                            &the_waiting,
                            &full_epi_name,
                            &episode_name,
                            wait_color,
                        );
                    }
                }
            }
        })
        .collect();
    text
}
