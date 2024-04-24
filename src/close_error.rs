use crate::app_state;
use crate::area_rects;
use crate::render_misc;
use crate::the_types;

use thousands::Separable;

#[allow(unused)]
use log::{debug, info, trace, warn};
use ratatui::{prelude::*, widgets::*};

pub fn draw_close_all(console_frame: &mut Frame, a_message: String) {
    let area = console_frame.size();
    let paragraph = Paragraph::new(a_message)
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Download all episodes in this podcast"));

    let area = area_rects::centered_rect(60, 31, area);
    console_frame.render_widget(Clear, area);
    console_frame.render_widget(paragraph, area);
    let pop_close_area = area_rects::get_error_close_area(console_frame);
    render_misc::render_close(console_frame, pop_close_area, "Ok");
}

pub fn render_all_ok(console_frame: &mut Frame, the_app: &app_state::DownApp) {
    if the_app.ui_state == the_types::UiState::State201AllEpisodes {
        let mut download_bytes = 0;
        for (media_name, media_length) in &the_app.episode_2_len {
            if !the_app.local_episode_files.contains_key(media_name) {
                download_bytes += media_length;
            }
        }
        let bytes_commas = download_bytes.separate_with_commas();
        let all_mess: String;
        if bytes_commas == "0" {
            all_mess =
                "\n Are you sure?\n There are an unknown number of bytes to download.".to_string();
        } else {
            all_mess = format!(
                "\n Are you sure? There are {:?} bytes to download.",
                &bytes_commas
            );
        }
        draw_close_all(console_frame, all_mess);
    }
}

fn create_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Gray))
        .title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
}

pub fn render_error_close(console_frame: &mut Frame, the_app: &mut app_state::DownApp) {
    if the_app.ui_state == the_types::UiState::State301WaitForPopErrorClose {
        let the_err_mess = the_app.cur_error.clone();
        draw_close_popup(console_frame, the_err_mess);
    }
}

pub fn draw_close_popup(console_frame: &mut Frame, a_message: String) {
    let area = console_frame.size();
    let paragraph = Paragraph::new(a_message)
        .style(Style::default().fg(Color::Gray))
        .block(create_block("My Error"));
    let area = area_rects::centered_rect(60, 30, area);
    console_frame.render_widget(Clear, area);
    console_frame.render_widget(paragraph, area);
    let pop_close_area = area_rects::get_error_close_area(console_frame);
    render_misc::render_close(console_frame, pop_close_area, "Ok");
}
