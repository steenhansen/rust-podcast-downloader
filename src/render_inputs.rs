#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::app_state;
use crate::const_globals;
use crate::g_current_active;
use crate::the_types;

use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::style::Stylize;
use ratatui::{prelude::*, widgets::*};
use std::str;

pub fn render_box(
    draw_area: Rect,
    console_frame: &mut Frame,
    the_title: &str,
    draw_name: String,
    url_color: Color,
    is_edit: bool,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let box_style = Style::default().fg(url_color);
    let paragraph = Paragraph::new(draw_name.clone())
        .block(Block::new().title(the_title).borders(Borders::ALL))
        .style(box_style)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    console_frame.render_widget(paragraph, area_safe);

    if is_edit {
        let so_far_len = draw_name.len();
        let end_space = area_safe.x + so_far_len as u16;
        let white_square = Rect {
            x: end_space + 1,
            y: area_safe.y + 1,
            width: 1,
            height: 1,
        };
        let cursor = " ".bold().style(Style::new().black().on_white());
        console_frame.render_widget(cursor, white_square);
    }
}

pub fn render_url(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_state::DownApp,
    box_title: &str,
    wait_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let draw_name = the_app.new_podcast_url.clone();
    let has_chars = draw_name.len() > 0;

    let is_edit = the_app.ui_state == the_types::UiState::State001NewPodcastUrl;

    let mut url_color = wait_color;
    if wait_color == Color::Reset {
        if is_edit || has_chars {
            url_color = const_globals::NORMAL_TEXT_COL;
        } else {
            url_color = const_globals::NO_FOCUS_TEXT;
        }
    }

    render_box(
        area_safe,
        console_frame,
        box_title,
        draw_name,
        url_color,
        is_edit,
    );
}

pub fn render_name(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_state::DownApp,
    box_title: &str,
    wait_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let draw_name = the_app.new_podcast_name.clone();
    let is_edit = the_app.ui_state == the_types::UiState::State002NewPodcastName;
    let has_chars = draw_name.len() > 0;
    let mut url_color = wait_color;
    if wait_color == Color::Reset {
        if is_edit || has_chars {
            url_color = const_globals::NORMAL_TEXT_COL;
        } else {
            url_color = const_globals::NO_FOCUS_TEXT;
        }
    }
    render_box(
        area_safe,
        console_frame,
        box_title,
        draw_name,
        url_color,
        is_edit,
    );
}

pub fn render_add_podcast(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_state::DownApp,
    box_title: &str,
    wait_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());

    let mut add_text_color = Color::Gray;
    let mut add_background_color = wait_color;
    if wait_color == Color::Reset {
        if the_app.new_podcast_url.len() > 0 && the_app.new_podcast_name.len() > 0 {
            add_text_color = const_globals::NORMAL_TEXT_COL;
            add_background_color = const_globals::BUTTON_READY;
        } else {
            add_text_color = const_globals::DIMMED_BUTTON_TEXT;
            add_background_color = const_globals::BUTTON_OFF;
        }
    }

    let text_style = Style::default().fg(add_text_color);
    let background_style = Style::default().bg(add_background_color);

    let paragraph = Paragraph::new(box_title)
        .style(background_style)
        .block(Block::new().style(text_style));

    console_frame.render_widget(paragraph, area_safe);
}
pub fn render_all_podcast(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_state::DownApp,
    box_title: &str,
    wait_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());

    let mut all_text_color = Color::Gray;
    let mut all_background_color = wait_color;
    if wait_color == Color::Reset {
        if can_do_all(the_app) {
            all_text_color = const_globals::NORMAL_TEXT_COL;
            all_background_color = const_globals::BUTTON_READY;
        } else {
            all_text_color = const_globals::DIMMED_BUTTON_TEXT;
            all_background_color = const_globals::BUTTON_OFF;
        }
    }
    let text_style = Style::default().fg(all_text_color);
    let background_style = Style::default().bg(all_background_color);

    let paragraph = Paragraph::new(box_title)
        .style(background_style)
        .block(Block::new().style(text_style));

    console_frame.render_widget(paragraph, area_safe);
}

fn can_do_all(the_app: &mut app_state::DownApp) -> bool {
    let a_podcast_is_selected = the_app.selected_podcast.len() > 0;
    let currently_downloading = g_current_active::active_downloading() == 0;
    let local_file_count = the_app.local_episode_files.len();
    let episode_file_count = the_app.episode_2_url.len();
    let just_dones = g_current_active::just_done(the_app.selected_podcast.clone());
    let old_and_new_count = local_file_count + just_dones;
    let local_count_same_as_rss = old_and_new_count == episode_file_count;
    if a_podcast_is_selected && currently_downloading && !local_count_same_as_rss {
        return true;
    }

    let old_and_new_count = the_app.local_episode_files.len() + just_dones;
    let files_not_done = old_and_new_count < the_app.episode_2_url.len();

    if the_app.selected_podcast.len() > 0
        && g_current_active::active_downloading() == 0
        && files_not_done
    {
        return true;
    }
    false
}
