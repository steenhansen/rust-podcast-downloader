use crate::consts::area_rects;
use crate::consts::const_globals;
use crate::consts::the_types;
use crate::dialogs::render_misc;
use crate::state::app_state;

use thousands::Separable;

#[allow(unused)]
use log::{debug, info, trace, warn};
use ratatui::{prelude::*, widgets::*};

pub fn draw_help(console_frame: &mut Frame, the_app: &app_state::DownApp, a_message: String) {
    let area = console_frame.size();
    let paragraph = Paragraph::new(a_message)
        .style(Style::default().fg(Color::White))
        .block(create_block("Commands").fg(Color::White));

    let area = area_rects::centered_rect(60, 41, area);
    console_frame.render_widget(Clear, area);
    console_frame.render_widget(paragraph, area);
    let pop_close_area = area_rects::ok_dialog_area(console_frame);
    render_misc::render_close(console_frame, the_app, pop_close_area);
}

pub fn render_help(console_frame: &mut Frame, the_app: &app_state::DownApp) {
    if the_app.ui_state == the_types::UiState::State501Help {
        let help_1 = "           H : This help\n";
        let help_2 = "         Tab : Move to next text box\n";
        let help_3 = "      Crtl-C : Quit\n";
        let help_4 = "           ↑ : Scroll episodes up\n";
        let help_5 = "           ↓ : Scroll episodes down\n";
        let help_6 = " Scoll Wheel : Scroll episodes up & down\n";
        let help_mess = help_1.to_owned() + help_2 + help_3 + help_4 + help_5 + help_6;
        draw_help(console_frame, the_app, help_mess);
    }
}

fn create_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Gray))
        .title(Span::styled(title, Style::default()))
}

pub fn render_error_close(console_frame: &mut Frame, the_app: &mut app_state::DownApp) {
    if the_app.ui_state == the_types::UiState::State301WaitForPopErrorClose {
        let the_err_mess = the_app.cur_error.clone();
        draw_close_popup(console_frame, the_app, the_err_mess);
    }
}

pub fn draw_close_popup(
    console_frame: &mut Frame,
    the_app: &app_state::DownApp,
    a_message: String,
) {
    let area = console_frame.size();
    let paragraph = Paragraph::new(a_message)
        .style(Style::default().fg(Color::Gray))
        .block(create_block("My Error"));
    let area = area_rects::centered_rect(60, 41, area);
    console_frame.render_widget(Clear, area);
    console_frame.render_widget(paragraph, area);
    let pop_close_area = area_rects::ok_dialog_area(console_frame);
    render_misc::render_close(console_frame, the_app, pop_close_area);
}

///////////////////
// draw_are_you_sure

pub fn render_disk_space(console_frame: &mut Frame, the_app: &app_state::DownApp) {
    if the_app.ui_state == the_types::UiState::State201EveryEpisode {
        let mut download_bytes: i64 = 0;
        for (media_name, media_length) in &the_app.episode_2_len {
            if !the_app.local_episode_files.contains_key(media_name) {
                download_bytes += *media_length as i64;
            }
        }
        let bytes_commas = download_bytes.separate_with_commas();
        let every_mess: String;
        if bytes_commas == "0" {
            every_mess =
                "\n There are an unknown number of bytes to download, continue?".to_string();
        } else {
            every_mess = format!(
                "\n There are {:?} bytes to download, continue?",
                &bytes_commas
            );
        }
        draw_disk_space(console_frame, the_app, every_mess);
    }
}

pub fn draw_disk_space(console_frame: &mut Frame, the_app: &app_state::DownApp, a_message: String) {
    let area = console_frame.size();
    let paragraph = Paragraph::new(a_message)
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Download every episodes in this podcast"));

    let area = area_rects::centered_rect(60, 41, area);
    console_frame.render_widget(Clear, area);
    console_frame.render_widget(paragraph, area);

    ///////////////////

    let mut yes_text_color = const_globals::BTN_EVERY_TEXT_READY;
    let mut yes_background_color = const_globals::BTN_EVERY_BACK_READY;
    if the_app.hover_element == app_state::HOVER_YES_SURE {
        yes_text_color = const_globals::BTN_EVERY_TEXT_HOVER;
        yes_background_color = const_globals::BTN_EVERY_BACK_HOVER;
    }
    let yes_style = Style::default().fg(yes_text_color).bg(yes_background_color);

    /////
    let yes_close_area = area_rects::yes_are_sure_dialog_area(console_frame);
    let area_ok = yes_close_area.intersection(console_frame.size());
    let paragraph = Paragraph::new("\n  Yes")
        .block(Block::new().borders(Borders::NONE))
        .style(yes_style);
    console_frame.render_widget(paragraph, area_ok);
    ///////
    let mut no_text_color = const_globals::BTN_EVERY_TEXT_READY;
    let mut no_background_color = const_globals::BTN_EVERY_BACK_READY;
    if the_app.hover_element == app_state::HOVER_NO_SURE {
        no_text_color = const_globals::BTN_EVERY_TEXT_HOVER;
        no_background_color = const_globals::BTN_EVERY_BACK_HOVER;
    }
    let no_style = Style::default().fg(no_text_color).bg(no_background_color);

    let no_close_area = area_rects::no_are_sure_dialog_area(console_frame);
    let area_cancel = no_close_area.intersection(console_frame.size());
    let paragraph = Paragraph::new("\n  No")
        .block(Block::new().borders(Borders::NONE))
        .style(no_style);
    console_frame.render_widget(paragraph, area_cancel);
}
