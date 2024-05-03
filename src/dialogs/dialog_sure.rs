#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_areas;
use crate::consts::const_colors;
use crate::consts::const_types;
use crate::dialogs::dialog_render;
use crate::misc::misc_ui;
use crate::state::state_app;

use crossterm::event::MouseEvent;
use ratatui::{prelude::*, widgets::*};
use thousands::Separable;

pub fn sure_render(console_frame: &mut Frame, the_app: &state_app::DownApp) {
    if the_app.ui_state == const_types::UiState::State201EveryEpisode {
        let mut download_bytes: i64 = 0;
        for (media_name, chunks_length) in &the_app.episode_2_len {
            if !the_app.local_episode_files.contains_key(media_name) {
                download_bytes += *chunks_length as i64;
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
        ok_draw_disk_space(console_frame, the_app, every_mess);
    }
}

pub fn ok_draw_disk_space(
    console_frame: &mut Frame,
    the_app: &state_app::DownApp,
    a_message: String,
) {
    let area = console_frame.size();

    let mut border_color = const_colors::FOCUS_NOT;
    if the_app.hover_element == state_app::HOVER_SURE_DIALOG
        || the_app.hover_element == state_app::HOVER_SURE_YES
        || the_app.hover_element == state_app::HOVER_SURE_NO
    {
        border_color = const_colors::FOCUS_HOVER;
    }

    let paragraph = Paragraph::new(a_message)
        .style(Style::default().fg(border_color))
        .block(
            dialog_render::dialog_block("Download every episodes in this podcast").fg(border_color),
        );
    let help_size = const_areas::DIALOG_SURE_AREA;
    let centered_area = dialog_render::dialog_centered(help_size, area);
    console_frame.render_widget(Clear, centered_area);
    console_frame.render_widget(paragraph, centered_area);

    let area_yes = hover_sure_yes_area(console_frame);
    let hover_element = the_app.hover_element.clone();
    let the_paragraph = dialog_render::dialog_yes_no(
        hover_element,
        state_app::HOVER_SURE_YES.to_string(),
        "\n  Yes",
    );
    console_frame.render_widget(the_paragraph, area_yes);

    let area_no = hover_sure_no_area(console_frame);
    let hover_element = the_app.hover_element.clone();
    let the_paragraph = dialog_render::dialog_yes_no(
        hover_element,
        state_app::HOVER_SURE_NO.to_string(),
        "\n  No",
    );
    console_frame.render_widget(the_paragraph, area_no);
}

pub fn hover_sure_yes_area(console_frame: &mut Frame) -> Rect {
    let frame_area = console_frame.size();
    let help_size = const_areas::DIALOG_SURE_AREA;
    let help_centered = dialog_render::dialog_centered(help_size, frame_area);

    let ok_area = const_areas::DIALOG_SURE_YES;
    let ok_centered_area = dialog_render::ok_centered(ok_area, help_centered);
    ok_centered_area
}

pub fn hover_sure_no_area(console_frame: &mut Frame) -> Rect {
    let frame_area = console_frame.size();
    let help_size = const_areas::DIALOG_SURE_AREA;
    let help_centered = dialog_render::dialog_centered(help_size, frame_area);

    let ok_area = const_areas::DIALOG_SURE_NO;
    let ok_centered_area = dialog_render::ok_centered(ok_area, help_centered);
    ok_centered_area
}

pub fn sure_hover(
    console_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    hover_event: MouseEvent,
) {
    let frame_area = console_frame.size();
    let help_size = const_areas::HELP_SIZE_AREA;
    let help_centered = dialog_render::dialog_centered(help_size, frame_area);

    let column = hover_event.column;
    let row = hover_event.row;
    if misc_ui::rect_point_in(column, row, help_centered) {
        the_app.hover_element = state_app::HOVER_SURE_DIALOG.to_string();

        let hover_help_ok_area = hover_sure_yes_area(console_frame);
        let column = hover_event.column;
        let row = hover_event.row;

        if misc_ui::rect_point_in(column, row, hover_help_ok_area) {
            the_app.hover_element = state_app::HOVER_SURE_YES.to_string();
        }

        let hover_help_no_area = hover_sure_no_area(console_frame);
        let column = hover_event.column;
        let row = hover_event.row;

        if misc_ui::rect_point_in(column, row, hover_help_no_area) {
            the_app.hover_element = state_app::HOVER_SURE_NO.to_string();
        }
    }
}
