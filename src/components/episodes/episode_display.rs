#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::components::episodes::episode_case;
use crate::consts::const_colors;
use crate::consts::const_types;
use crate::state::state_app;

use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::style::Stylize;
use ratatui::widgets::*;

pub fn display_render_epi_list(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut state_app::DownApp,
    box_title: String,
    wait_color: Color,
) {
    let ordered_episodes = the_app.ordered_episodes.clone();
    let local_episode_files = the_app.local_episode_files.clone();
    let colored_epi_rows: Vec<Line> = episode_case::case_colored_episodes(
        ordered_episodes,
        local_episode_files,
        the_app,
        wait_color,
    );
    let episode_title = format!("{}——{}", the_app.selected_podcast, box_title);
    let area_safe = draw_area.intersection(console_frame.size());
    the_app.state_scroll_podcasts = the_app
        .state_scroll_podcasts
        .content_length(colored_epi_rows.len());

    let mut episode_border_color = wait_color;
    if wait_color == Color::Reset {
        if the_app.hover_element == state_app::HOVER_EPISODES {
            episode_border_color = const_colors::EPISODES_HOVER;
        } else {
            episode_border_color = const_colors::EPISODES_READY;
        }
    }

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

pub fn display_display_render_epi_list_empty(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut state_app::DownApp,
    wait_color: Color,
) {
    let episode_title = format!("{}", the_app.selected_podcast);
    let episode_text_color = match wait_color {
        Color::Reset => const_colors::EPISODE_LOADING,
        _ => wait_color,
    };
    let ui_state = the_app.ui_state;

    let waiting_mess: String;
    if ui_state == const_types::UiState::State101ReadingRss
        || ui_state == const_types::UiState::State102ShowWaiting
    {
        waiting_mess = "loading...".to_owned();
    } else if ui_state == const_types::UiState::State601KillingDownloads {
        waiting_mess = "killing...".to_owned();
    } else {
        waiting_mess = "".to_owned();
    }

    let loading_files = Span::styled(waiting_mess, Style::default().fg(episode_text_color));
    let colored_epi_rows: Vec<Line> = [Line::from(loading_files)].to_vec();

    let area_safe = draw_area.intersection(console_frame.size());
    let episode_border_color = match wait_color {
        Color::Reset => const_colors::EPISODES_READY,
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
