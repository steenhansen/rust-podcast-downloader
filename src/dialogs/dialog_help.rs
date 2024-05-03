#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::dialogs::dialog_render;

use crate::consts::consts_areas;
use crate::consts::consts_rects;
use crate::consts::consts_types;

use crate::state::state_app;

use crossterm::event::MouseEvent;
use ratatui::{prelude::*, widgets::*};

pub fn help_render(console_frame: &mut Frame, the_app: &state_app::DownApp) {
    if the_app.ui_state == consts_types::UiState::State501Help {
        let help_1 = "                 H : This help\n";
        let help_2 = "               Tab : Move to next text box\n";
        let help_3 = "            Crtl-C : Quit\n";
        let help_4 = "       ↑ & Page Up : Scroll episodes up\n";
        let help_5 = "     ↓ & Page Down : Scroll episodes down\n";
        let help_6 = "\n";
        let help_7 = " Mouse Scoll Wheel : Scroll episodes up & down\n";
        let help_mess = help_1.to_owned() + help_2 + help_3 + help_4 + help_5 + help_6 + help_7;
        ok_draw_help(console_frame, the_app, help_mess);
    }
}

pub fn ok_draw_help(console_frame: &mut Frame, the_app: &state_app::DownApp, a_message: String) {
    let area = console_frame.size();
    let paragraph = Paragraph::new(a_message)
        .style(Style::default().fg(Color::White))
        .block(dialog_render::dialog_block("Commands").fg(Color::White));
    let help_size = consts_areas::HELP_SIZE_AREA;
    let centered_area = dialog_render::dialog_centered(help_size, area);
    console_frame.render_widget(Clear, centered_area);
    console_frame.render_widget(paragraph, centered_area);

    let area_yes = hover_help_ok_area(console_frame);
    let hover_element = the_app.hover_element.clone();
    let the_paragraph = dialog_render::dialog_yes_no(
        hover_element,
        state_app::HOVER_HELP_OK_DIALOG.to_string(),
        "\n  Ok",
    );
    console_frame.render_widget(the_paragraph, area_yes);
}

pub fn hover_help_ok_area(console_frame: &mut Frame) -> Rect {
    let frame_area = console_frame.size();
    let help_size = consts_areas::HELP_SIZE_AREA;
    let help_centered = dialog_render::dialog_centered(help_size, frame_area);

    let ok_area = consts_areas::HELP_SIZE_OK;
    let ok_centered_area = dialog_render::ok_centered(ok_area, help_centered);
    ok_centered_area
}

pub fn help_hover(
    console_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    hover_event: MouseEvent,
) {
    let hover_help_ok_area = hover_help_ok_area(console_frame);
    let column = hover_event.column;
    let row = hover_event.row;

    if consts_rects::rect_point_in(column, row, hover_help_ok_area) {
        the_app.hover_element = state_app::HOVER_HELP_OK_DIALOG.to_string();
    }
}
