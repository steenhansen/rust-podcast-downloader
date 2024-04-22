#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::app_state;
use crate::const_globals;

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
) {
    let ordered_episodes = the_app.ordered_episodes.clone();
    let local_episode_files = the_app.local_episode_files.clone();
    let colored_epi_rows: Vec<Line> =
        colored_episodes(ordered_episodes, local_episode_files, the_app);
    let episode_title = format!("{}——{}", the_app.selected_podcast, box_title);
    let area_safe = draw_area.intersection(console_frame.size());
    the_app.state_scroll_podcasts = the_app
        .state_scroll_podcasts
        .content_length(colored_epi_rows.len());
    let create_block = |title: String| Block::bordered().gray().title(title.bold());
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
) {
    let episode_title = format!("{}", the_app.selected_podcast);
    let colored_epi_rows: Vec<Line> = [Line::from("loading...".red())].to_vec();
    let area_safe = draw_area.intersection(console_frame.size());
    the_app.state_scroll_podcasts = the_app
        .state_scroll_podcasts
        .content_length(colored_epi_rows.len());
    let create_block = |title: String| Block::bordered().gray().title(title.bold());
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
                let old_local_file = color_old_epi(&episode_name);
                return Line::from(old_local_file);
            } else {
                let cur_read_status = g_current_active::G_CURRENT_ACTIVE
                    .lock()
                    .expect("memory err");
                let full_epi_name = format!("{selected_podcast}/{episode_name}");
                match cur_read_status.get(&full_epi_name) {
                    Some(num_bytes) => {
                        return start_or_downloading_or_done(&num_bytes, &episode_name);
                    }
                    None => {
                        return possible_or_waiting(&the_waiting, &full_epi_name, &episode_name);
                    }
                }
            }
        })
        .collect();
    text
}

fn start_or_downloading_or_done(num_bytes: &str, episode_name: &str) -> Line<'static> {
    if num_bytes == const_globals::DOWNLOADED_MEDIA {
        let new_local_file = color_finished_epi(&episode_name);
        return Line::from(new_local_file);
    } else {
        if num_bytes == "0" {
            let await_start = color_start_epi(&episode_name);
            return Line::from(await_start);
        } else {
            let downloading_file = color_continue_epi(num_bytes, &episode_name);
            return Line::from(downloading_file);
        }
    }
}

fn possible_or_waiting(
    the_waiting: &HashMap<String, String>,
    full_epi_name: &str,
    episode_name: &str,
) -> Line<'static> {
    if the_waiting.contains_key(full_epi_name) {
        let waiting_file = color_waiting_epi(&episode_name);
        return Line::from(waiting_file);
    } else {
        let possible_file = color_possible_epi(&episode_name);
        return Line::from(possible_file);
    }
}

fn color_old_epi(episode_name: &str) -> Span<'static> {
    let indented_old = const_globals::FINISHED_INDENT.to_owned() + episode_name;
    let old_local_file = Span::styled(
        indented_old,
        Style::default().fg(const_globals::OLD_LOCAL_EPISODE),
    );
    old_local_file
}

fn color_finished_epi(episode_name: &str) -> Span<'static> {
    let just_finished = const_globals::FINISHED_INDENT.to_owned() + episode_name;
    let new_local_file = Span::styled(
        just_finished,
        Style::default().fg(const_globals::JUST_GOT_EPISODE),
    );
    new_local_file
}

fn color_start_epi(episode_name: &str) -> Span<'static> {
    let new_f_pos = const_globals::ZERO_START_INDENT.to_owned() + episode_name;
    let new_local_file = Span::styled(
        new_f_pos,
        Style::default()
            .fg(const_globals::START_EPISODE_FG)
            .bg(const_globals::START_EPISODE_BG),
    );
    new_local_file
}

fn color_continue_epi(num_bytes: &str, episode_name: &str) -> Span<'static> {
    let new_f_pos = num_bytes.to_owned() + " - " + episode_name;
    let new_local_file = Span::styled(
        new_f_pos,
        Style::default().fg(const_globals::ACTIVE_EPISODE),
    );
    new_local_file
}

fn color_waiting_epi(episode_name: &str) -> Span<'static> {
    let waiting_file = Span::styled(
        episode_name.to_owned(),
        Style::default().fg(const_globals::WAITING_EPISODE),
    );
    waiting_file
}

fn color_possible_epi(episode_name: &str) -> Span<'static> {
    let possible_file = Span::styled(
        episode_name.to_owned(),
        Style::default().fg(const_globals::CAN_DOWNLOAD_EPISODE),
    );
    possible_file
}
