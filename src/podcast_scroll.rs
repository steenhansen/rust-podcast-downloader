#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::the_types;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::app_ui;
use crate::const_globals;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::{prelude::*, widgets::*};

use std::io;

pub fn flip_prefix(the_app: &mut app_ui::DownApp) -> Result<(), io::Error> {
    let selected_podcast = the_app.selected_podcast.clone();
    let is_int_prefix = is_int_prefix(const_globals::ROOT_DIR, &selected_podcast);
    if is_int_prefix {
        let f_name = format!("{}/{}", selected_podcast, const_globals::INT_PREFIX_Y_N);
        fs::remove_file(f_name)?;
    } else {
        let f_name = format!("{}/{}", selected_podcast, const_globals::INT_PREFIX_Y_N);
        fs::write(f_name, "")?;
    }
    Ok(())
}

pub fn is_int_prefix(root_dir: &str, pod_name: &String) -> bool {
    let prefix_file_path = format!("{}{}/{}", root_dir, pod_name, const_globals::INT_PREFIX_Y_N);
    if Path::new(&prefix_file_path).is_file() {
        return true;
    }
    false
}

pub fn read_podcast_dir(root_dir: &str) -> HashMap<String, String> {
    let mut episodes_local: HashMap<String, String> = HashMap::new();
    let dir_entries = fs::read_dir(root_dir).unwrap();
    for an_entry in dir_entries {
        let the_entry = an_entry.expect("bard2");
        let the_path = the_entry.path();
        if the_path.is_dir() {
            let my_str = the_path.as_path().display().to_string();
            let pod_name: String = my_str.chars().skip(2).collect();
            let rss_file_path =
                format!("{}{}/{}", root_dir, pod_name, const_globals::RSS_TEXT_FILE);

            if Path::new(&rss_file_path).is_file() {
                let mut file_handle = File::open(&rss_file_path).unwrap();
                let mut pod_url = String::new();
                file_handle.read_to_string(&mut pod_url).unwrap();
                episodes_local.insert(pod_name, pod_url);
            }
        }
    }
    episodes_local
}

fn create_new_podcast(podcast_name: &str, contents_url: &str) -> Result<(), io::Error> {
    fs::create_dir(podcast_name)?;
    let f_name = format!("{}/{}", podcast_name, const_globals::RSS_TEXT_FILE);
    fs::write(f_name, contents_url)?;
    let f_name = format!("{}/{}", podcast_name, const_globals::INT_PREFIX_Y_N);
    fs::write(f_name, "")?;
    Ok(())
}

pub fn create_pod_dir(the_app: &mut app_ui::DownApp) -> Result<(), io::Error> {
    the_app.ui_state = the_types::UiState::StateNoFocus;
    create_new_podcast(&the_app.podcast_name, &the_app.podcast_url)?;
    the_app.podcast_name = String::from("");
    the_app.podcast_url = String::from("");
    get_dirs_of_podcasts(the_app);
    Ok(())
}

pub fn podcasts_vscroll(console_frame: &mut Frame, draw_area: Rect, the_app: &mut app_ui::DownApp) {
    let area_safe = draw_area.intersection(console_frame.size());
    console_frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        area_safe,
        &mut the_app.state_scroll_podcasts,
    );
}

pub fn get_dirs_of_podcasts(the_app: &mut app_ui::DownApp) {
    let unordered_podcasts = read_podcast_dir(const_globals::ROOT_DIR);
    the_app.ordered_podcasts = <HashMap<String, String> as Clone>::clone(&unordered_podcasts)
        .into_iter()
        .map(|(p_name, _p_url)| p_name)
        .collect();
    the_app
        .ordered_podcasts
        .sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
}

pub fn render_pod_list(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut app_ui::DownApp,
    box_title: String,
) {
    let area_safe = draw_area.intersection(console_frame.size());

    let ordered_podcasts = the_app.ordered_podcasts.clone();
    let selected_podcast = the_app.selected_podcast.clone();
    let colored_pod_rows: Vec<Line> = colored_podcasts(ordered_podcasts, selected_podcast);
    the_app.state_scroll_podcasts = the_app
        .state_scroll_podcasts
        .content_length(colored_pod_rows.len());

    let create_block = |title: String| Block::bordered().gray().title(title.bold());
    let paragraph = Paragraph::new(colored_pod_rows.clone())
        .green()
        .block(create_block(box_title))
        .scroll((the_app.scrolled_podcasts as u16, 0));
    console_frame.render_widget(paragraph, area_safe);
}

pub fn colored_podcasts(
    ordered_podcastsed: Vec<String>,
    selected_podcast: String,
) -> Vec<Line<'static>> {
    let text: Vec<Line> = ordered_podcastsed
        .into_iter()
        .map(|p_name| {
            if p_name == selected_podcast {
                Line::from(p_name.red())
            } else {
                Line::from(p_name)
            }
        })
        .collect();

    text
}
