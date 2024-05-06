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

pub fn error_render(console_frame: &mut Frame, the_app: &state_app::DownApp) {
    if the_app.ui_state == const_types::UiState::State301WaitForPopErrorClose {
        let the_err_mess = the_app.cur_error.clone();
        ok_draw_error(console_frame, the_app, the_err_mess);
    }
}

pub fn ok_draw_error(console_frame: &mut Frame, the_app: &state_app::DownApp, a_message: String) {
    let area = console_frame.size();

    let mut border_color = const_colors::FOCUS_NOT;
    if the_app.hover_element == state_app::HOVER_ERROR_DIALOG
        || the_app.hover_element == state_app::HOVER_ERROR_OK
    {
        border_color = const_colors::FOCUS_HOVER;
    }

    let paragraph = Paragraph::new(a_message)
        .style(Style::default().fg(border_color))
        .block(dialog_render::dialog_block("My Error").fg(border_color));
    let error_size = const_areas::ERROR_SIZE_AREA;
    let centered_area = dialog_render::dialog_centered(error_size, area);
    let area_safe = centered_area.intersection(console_frame.size());
    console_frame.render_widget(Clear, area_safe);
    console_frame.render_widget(paragraph, area_safe);

    let area_ok = hover_error_ok_area(console_frame);
    let hover_element = the_app.hover_element.clone();
    let the_paragraph = dialog_render::dialog_yes_no(
        hover_element,
        state_app::HOVER_ERROR_OK.to_string(),
        "\n  Ok",
    );
    let ok_safe = area_ok.intersection(console_frame.size());
    console_frame.render_widget(the_paragraph, ok_safe);
}

pub fn hover_error_ok_area(console_frame: &mut Frame) -> Rect {
    let frame_area = console_frame.size();
    let error_size = const_areas::ERROR_SIZE_AREA;
    let error_centered = dialog_render::dialog_centered(error_size, frame_area);

    let ok_area = const_areas::ERROR_SIZE_OK;
    let ok_centered_area = dialog_render::ok_centered(ok_area, error_centered);
    ok_centered_area
}

pub fn error_hover(
    console_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    hover_event: MouseEvent,
) {
    let frame_area = console_frame.size();
    let error_size = const_areas::ERROR_SIZE_AREA;
    let error_centered = dialog_render::dialog_centered(error_size, frame_area);

    let column = hover_event.column;
    let row = hover_event.row;

    let error_safe = error_centered.intersection(console_frame.size());
    if misc_ui::rect_point_in(column, row, error_safe) {
        the_app.hover_element = state_app::HOVER_ERROR_DIALOG.to_string();

        let area_ok = hover_error_ok_area(console_frame);

        let ok_safe = area_ok.intersection(console_frame.size());
        if misc_ui::rect_point_in(column, row, ok_safe) {
            the_app.hover_element = state_app::HOVER_ERROR_OK.to_string();
        }
    }
}
