use crate::const_globals;
//             //https://docs.rs/ratatui/latest/src/tabs/tabs.rs.html#144
use crate::down_app;

use crate::file_status;
#[allow(unused)]
use log::{debug, info, trace, warn};
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::{prelude::*, widgets::*};
use std::collections::HashMap;

pub fn episode_vscroll(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut down_app::DownApp,
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
    the_app: &mut down_app::DownApp,
    box_title: String,
) {
    let ordered_episodes = the_app.ordered_episodes.clone();
    let local_episode_files = the_app.local_episode_files.clone();
    let colored_epi_rows: Vec<Line> =
        colored_episodes(ordered_episodes, local_episode_files, the_app);
    let episode_title = format!("{}——{}", the_app.selected_podcast, box_title);
    ////////////
    let area_safe = draw_area.intersection(console_frame.size());
    the_app.state_scroll_podcasts = the_app
        .state_scroll_podcasts
        .content_length(colored_epi_rows.len());
    let create_block = |title: String| Block::bordered().gray().title(title.bold());
    let paragraph = Paragraph::new(colored_epi_rows.clone())
        .green()
        .block(create_block(episode_title))
        .scroll((the_app.scrolled_episodes as u16, 0));
    console_frame.render_widget(paragraph, area_safe);
}

pub fn render_epi_list_empty(
    console_frame: &mut Frame,
    draw_area: Rect,
    the_app: &mut down_app::DownApp,
    _box_title: String,
) {
    let episode_title = format!("{}", the_app.selected_podcast);
    let colored_epi_rows: Vec<Line> = [Line::from("loading...".red())].to_vec();
    //////////
    let area_safe = draw_area.intersection(console_frame.size());
    the_app.state_scroll_podcasts = the_app
        .state_scroll_podcasts
        .content_length(colored_epi_rows.len());
    let create_block = |title: String| Block::bordered().gray().title(title.bold());
    let paragraph = Paragraph::new(colored_epi_rows.clone())
        .green()
        .block(create_block(episode_title))
        .scroll((the_app.scrolled_episodes as u16, 0));
    console_frame.render_widget(paragraph, area_safe);
}

fn colored_episodes(
    ordered_episodes: Vec<String>,
    local_episode_files: HashMap<String, String>,
    the_app: &mut down_app::DownApp,
) -> Vec<Line<'static>> {
    let text: Vec<Line> = ordered_episodes
        .into_iter()
        .map(|p_name| {
            if local_episode_files.contains_key(&p_name) {
                Line::from(p_name.dark_gray())
            } else {
                let cur_read_status = file_status::G_SS.lock().unwrap();
                let selected_podcast = &the_app.selected_podcast;
                let full_epi_name = format!("{selected_podcast}/{p_name}");
                //https://ratatui.rs/how-to/render/style-text/
                match cur_read_status.get(&full_epi_name) {
                    Some(num_bytes) => {
                        if num_bytes == const_globals::DOWNLOADED_MEDIA {
                            Line::from(p_name.cyan())
                        } else {
                            if num_bytes == "0" {
                                let new_f_pos = num_bytes.to_owned() + "0000000 - " + &p_name;
                                Line::from(new_f_pos.green())
                            } else {
                                let new_f_pos = num_bytes.to_owned() + " - " + &p_name;
                                Line::from(new_f_pos.green())
                            }
                        }
                    }
                    None => Line::from(p_name.green()),
                }
            }
        })
        .collect();

    text
}
