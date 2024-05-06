#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_areas;
use crate::consts::const_colors;
use crate::consts::const_types;
use crate::dialogs::dialog_render;
use crate::misc::misc_ui;
use crate::state::state_app;

use crossterm::event::MouseEvent;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::{prelude::*, widgets::*};

pub fn help_render(console_frame: &mut Frame, the_app: &state_app::DownApp) {
    if the_app.ui_state == const_types::UiState::State501Help {
        let help_1 = "                 H : This help\n";
        let help_2 = "               Tab : Move to next text box\n";
        let help_3 = "            Crtl-C : Quit\n";
        let help_4 = "            Crtl-V : Paste text into text input\n";
        let help_5 = "       ↑ & Page Up : Scroll episodes up\n";
        let help_6 = "     ↓ & Page Down : Scroll episodes down\n";
        let help_7 = "\n";
        let help_8 = " Mouse Scoll Wheel : Scroll episodes up & down\n";
        let help_mess =
            help_1.to_owned() + help_2 + help_3 + help_4 + help_5 + help_6 + help_7 + help_8;
        ok_draw_help(console_frame, the_app, help_mess);
    }
}

pub fn ok_draw_help(console_frame: &mut Frame, the_app: &state_app::DownApp, a_message: String) {
    let area = console_frame.size();

    let mut border_color = const_colors::FOCUS_NOT;
    if the_app.hover_element == state_app::HOVER_HELP_DIALOG
        || the_app.hover_element == state_app::HOVER_HELP_OK
    {
        border_color = const_colors::FOCUS_HOVER;
    }

    let paragraph = Paragraph::new(a_message)
        .style(Style::default().fg(border_color))
        .block(dialog_render::dialog_block("Commands").fg(border_color));
    let help_size = const_areas::HELP_SIZE_AREA;
    let centered_area = dialog_render::dialog_centered(help_size, area);

    let area_safe = centered_area.intersection(console_frame.size());

    console_frame.render_widget(Clear, area_safe);
    console_frame.render_widget(paragraph, area_safe);

    let area_ok = hover_help_ok_area(console_frame);
    let ok_safe = area_ok.intersection(console_frame.size());
    let hover_element = the_app.hover_element.clone();
    let the_paragraph = dialog_render::dialog_yes_no(
        hover_element,
        state_app::HOVER_HELP_OK.to_string(),
        "\n  Ok",
    );
    console_frame.render_widget(the_paragraph, ok_safe);
}

pub fn hover_help_ok_area(console_frame: &mut Frame) -> Rect {
    let frame_area = console_frame.size();
    let help_size = const_areas::HELP_SIZE_AREA;
    let help_centered = dialog_render::dialog_centered(help_size, frame_area);

    let ok_area = const_areas::HELP_SIZE_OK;
    let ok_centered_area = dialog_render::ok_centered(ok_area, help_centered);
    ok_centered_area
}

pub fn help_hover(
    console_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    hover_event: MouseEvent,
) {
    let frame_area = console_frame.size();
    let help_size = const_areas::HELP_SIZE_AREA;
    let help_centered = dialog_render::dialog_centered(help_size, frame_area);

    let column = hover_event.column;
    let row = hover_event.row;
    let area_safe = help_centered.intersection(console_frame.size());
    if misc_ui::rect_point_in(column, row, area_safe) {
        the_app.hover_element = state_app::HOVER_HELP_DIALOG.to_string();

        let area_ok = hover_help_ok_area(console_frame);
        if misc_ui::rect_point_in(column, row, area_ok) {
            the_app.hover_element = state_app::HOVER_HELP_OK.to_string();
        }
    }
}
