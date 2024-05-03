#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::consts_areas;

use crate::consts::consts_rects;
use crate::consts::consts_types;
use crate::dialogs::dialog_render;

use crate::state::state_app;

use crossterm::event::MouseEvent;
use ratatui::{prelude::*, widgets::*};
use thousands::Separable;

pub fn sure_render(console_frame: &mut Frame, the_app: &state_app::DownApp) {
    if the_app.ui_state == consts_types::UiState::State201EveryEpisode {
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
    let paragraph = Paragraph::new(a_message)
        .style(Style::default().fg(Color::White))
        .block(
            dialog_render::dialog_block("Download every episodes in this podcast").fg(Color::White),
        );
    let help_size = consts_areas::DIALOG_SURE_AREA;
    let centered_area = dialog_render::dialog_centered(help_size, area);
    console_frame.render_widget(Clear, centered_area);
    console_frame.render_widget(paragraph, centered_area);

    let area_yes = hover_sure_yes_area(console_frame);
    let hover_element = the_app.hover_element.clone();
    let the_paragraph = dialog_render::dialog_yes_no(
        hover_element,
        state_app::HOVER_YES_SURE.to_string(),
        "\n  Yes",
    );
    console_frame.render_widget(the_paragraph, area_yes);

    let area_no = hover_sure_no_area(console_frame);
    let hover_element = the_app.hover_element.clone();
    let the_paragraph = dialog_render::dialog_yes_no(
        hover_element,
        state_app::HOVER_NO_SURE.to_string(),
        "\n  No",
    );
    console_frame.render_widget(the_paragraph, area_no);
}

pub fn hover_sure_yes_area(console_frame: &mut Frame) -> Rect {
    let frame_area = console_frame.size();
    let help_size = consts_areas::DIALOG_SURE_AREA;
    let help_centered = dialog_render::dialog_centered(help_size, frame_area);

    let ok_area = consts_areas::DIALOG_SURE_YES;
    let ok_centered_area = dialog_render::ok_centered(ok_area, help_centered);
    ok_centered_area
}

pub fn hover_sure_no_area(console_frame: &mut Frame) -> Rect {
    let frame_area = console_frame.size();
    let help_size = consts_areas::DIALOG_SURE_AREA;
    let help_centered = dialog_render::dialog_centered(help_size, frame_area);

    let ok_area = consts_areas::DIALOG_SURE_NO;
    let ok_centered_area = dialog_render::ok_centered(ok_area, help_centered);
    ok_centered_area
}

pub fn sure_hover(
    console_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    hover_event: MouseEvent,
) {
    let hover_help_ok_area = hover_sure_yes_area(console_frame);
    let column = hover_event.column;
    let row = hover_event.row;

    if consts_rects::rect_point_in(column, row, hover_help_ok_area) {
        the_app.hover_element = state_app::HOVER_YES_SURE.to_string();
    }

    ////////////
    let hover_help_no_area = hover_sure_no_area(console_frame);
    let column = hover_event.column;
    let row = hover_event.row;

    if consts_rects::rect_point_in(column, row, hover_help_no_area) {
        the_app.hover_element = state_app::HOVER_NO_SURE.to_string();
    }
}
