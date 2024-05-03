#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_areas;
use crate::consts::const_colors;
use crate::consts::const_types;
use crate::globals::g_active;
use crate::misc::misc_ui;
use crate::state::state_app;

use crossterm::event::MouseEvent;
use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::{prelude::*, widgets::*};
use std::str;

pub fn every_show(console_frame: &mut Frame, the_app: &mut state_app::DownApp) {
    every_episode_rendered(
        console_frame,
        const_areas::EVERY_EPISODE_AREA,
        the_app,
        "\n Download Every Episode",
    );
}

pub fn every_hover(the_app: &mut state_app::DownApp, hover_event: MouseEvent) {
    let column = hover_event.column;
    let row = hover_event.row;
    if misc_ui::rect_point_in(column, row, const_areas::EVERY_EPISODE_AREA) {
        the_app.hover_element = state_app::HOVER_BTN_EVERY.to_string();
    }
}

// thus when enter inputs we must remember what state we were in

// no if we enter input, then we lose_focuse on episodes !!!!

pub fn every_active(the_app: &mut state_app::DownApp) -> bool {
    if the_app.ui_state == const_types::UiState::State103ShowEpisodes {
        let a_podcast_is_selected = the_app.selected_podcast.len() > 0;
        let local_file_count = the_app.local_episode_files.len();
        let episode_file_count = the_app.episode_2_url.len();
        let just_dones = g_active::active_just_done(the_app.selected_podcast.clone());
        let old_and_new_count = local_file_count + just_dones;
        let local_count_same_as_rss = old_and_new_count == episode_file_count;
        if a_podcast_is_selected && !local_count_same_as_rss {
            return true;
        }
        let old_and_new_count = the_app.local_episode_files.len() + just_dones;
        let files_not_done = old_and_new_count < the_app.episode_2_url.len();

        if the_app.selected_podcast.len() > 0
            && g_active::active_downloading() == 0
            && files_not_done
        {
            return true;
        }
    }
    false
}

pub fn every_clicked(the_app: &mut state_app::DownApp, the_click: MouseEvent) {
    if every_active(the_app) {
        let column = the_click.column;
        let row = the_click.row;
        if misc_ui::rect_point_in(column, row, const_areas::EVERY_EPISODE_AREA) {
            if the_app.selected_podcast.len() > 0 {
                the_app.ui_state = const_types::UiState::State201EveryEpisode;
            }
        }
    }
}

pub fn every_episode_rendered(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut state_app::DownApp,
    box_title: &str,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let mut every_text_color = const_colors::BTN_EVERY_TEXT_DEAD;
    let mut every_background_color = const_colors::BTN_EVERY_BACK_DEAD;

    if the_app.ui_state == const_types::UiState::State401DownloadPaused {
        every_text_color = const_colors::PAUSE_TEXT_COLOR;
        every_background_color = const_colors::PAUSE_BACK_COLOR;
    } else if every_active(the_app) {
        if the_app.hover_element == state_app::HOVER_BTN_EVERY {
            every_text_color = const_colors::BTN_EVERY_TEXT_HOVER;
            every_background_color = const_colors::BTN_EVERY_BACK_HOVER;
        } else {
            every_text_color = const_colors::BTN_EVERY_TEXT_READY;
            every_background_color = const_colors::BTN_EVERY_BACK_READY;
        }
    }

    let text_style = Style::default().fg(every_text_color);
    let background_style = Style::default().bg(every_background_color);

    let paragraph = Paragraph::new(box_title)
        .style(background_style)
        .block(Block::new().style(text_style));

    console_frame.render_widget(paragraph, area_safe);
}
