#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::app_state;
use crate::const_globals;
use crate::the_types;

use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::{prelude::*, widgets::*};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub fn read_podcast_dir(root_dir: &str) -> HashMap<String, String> {
    let mut episodes_local: HashMap<String, String> = HashMap::new();
    let dir_entries = fs::read_dir(root_dir).expect("bad-dir-read-err");
    for an_entry in dir_entries {
        let the_entry = an_entry.expect("bad-dir-entry-err");
        let the_path = the_entry.path();
        if the_path.is_dir() {
            let my_str = the_path.as_path().display().to_string();
            let pod_name: String = my_str.chars().skip(2).collect();
            let rss_file_path =
                format!("{}{}/{}", root_dir, pod_name, const_globals::RSS_TEXT_FILE);
            if Path::new(&rss_file_path).is_file() {
                let mut file_handle = File::open(&rss_file_path).expect("file-open-err");
                let mut pod_url = String::new();
                file_handle
                    .read_to_string(&mut pod_url)
                    .expect("read-to-string-err");
                episodes_local.insert(pod_name, pod_url);
            }
        }
    }
    episodes_local
}

fn create_new_podcast(new_podcast_name: &str, contents_url: &str) -> Result<(), io::Error> {
    fs::create_dir(new_podcast_name).expect("create-dir-err");
    let f_name = format!("{}/{}", new_podcast_name, const_globals::RSS_TEXT_FILE);
    fs::write(f_name, contents_url).expect("create-new-write-err");
    Ok(())
}

pub fn create_pod_dir(the_app: &mut app_state::DownApp) -> Result<(), io::Error> {
    the_app.ui_state = the_types::UiState::StateNoFocus;
    create_new_podcast(&the_app.new_podcast_name, &the_app.new_podcast_url).expect("make-dir-err");
    the_app.new_podcast_name = String::from("");
    the_app.new_podcast_url = String::from("");
    get_dirs_of_podcasts(the_app);
    Ok(())
}

pub fn podcasts_vscroll(
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
        &mut the_app.state_scroll_podcasts,
    );
}

pub fn get_dirs_of_podcasts(the_app: &mut app_state::DownApp) {
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
    the_app: &mut app_state::DownApp,
    box_title: String,
    wait_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());

    let ordered_podcasts = the_app.ordered_podcasts.clone();
    let selected_podcast = the_app.selected_podcast.clone();
    let colored_pod_rows: Vec<Line> =
        colored_podcasts(ordered_podcasts, selected_podcast, wait_color);
    the_app.state_scroll_podcasts = the_app
        .state_scroll_podcasts
        .content_length(colored_pod_rows.len());

    let podcast_border_color = match wait_color {
        Color::Reset => const_globals::NORMAL_BORDER_COL,
        _ => wait_color,
    };

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

pub fn colored_podcasts(
    ordered_podcastsed: Vec<String>,
    selected_podcast: String,
    wait_color: Color,
) -> Vec<Line<'static>> {
    let text: Vec<Line> = ordered_podcastsed
        .into_iter()
        .map(|podcast_name| {
            let mut podcast_text_color = wait_color;
            if wait_color == Color::Reset {
                if podcast_name == selected_podcast {
                    podcast_text_color = const_globals::PODCAST_SELECTED;
                } else {
                    podcast_text_color = const_globals::PODCAST_NOT_SELECTED;
                }
            }
            Line::styled(podcast_name, Style::default().fg(podcast_text_color))
        })
        .collect();

    text
}
