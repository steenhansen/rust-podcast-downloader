#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::consts_areas;
use crate::consts::consts_globals;
use crate::consts::consts_types;
use crate::dialogs::dialog_render;
use crate::misc::misc_ui;
use crate::state::state_app;

use crossterm::event::MouseEvent;
use ratatui::{prelude::*, widgets::*};

pub fn error_render(console_frame: &mut Frame, the_app: &state_app::DownApp) {
    if the_app.ui_state == consts_types::UiState::State301WaitForPopErrorClose {
        let the_err_mess = the_app.cur_error.clone();
        ok_draw_error(console_frame, the_app, the_err_mess);
    }
}

pub fn ok_draw_error(console_frame: &mut Frame, the_app: &state_app::DownApp, a_message: String) {
    let area = console_frame.size();

    let mut border_color = consts_globals::FOCUS_NOT;
    if the_app.hover_element == state_app::HOVER_ERROR_DIALOG
        || the_app.hover_element == state_app::HOVER_ERROR_OK
    {
        border_color = consts_globals::FOCUS_HOVER;
    }

    let paragraph = Paragraph::new(a_message)
        .style(Style::default().fg(border_color))
        .block(dialog_render::dialog_block("My Error").fg(border_color));
    let error_size = consts_areas::ERROR_SIZE_AREA;
    let centered_area = dialog_render::dialog_centered(error_size, area);
    console_frame.render_widget(Clear, centered_area);
    console_frame.render_widget(paragraph, centered_area);

    let area_ok = hover_error_ok_area(console_frame);
    let hover_element = the_app.hover_element.clone();
    let the_paragraph = dialog_render::dialog_yes_no(
        hover_element,
        state_app::HOVER_ERROR_OK.to_string(),
        "\n  Ok",
    );
    console_frame.render_widget(the_paragraph, area_ok);
}

pub fn hover_error_ok_area(console_frame: &mut Frame) -> Rect {
    let frame_area = console_frame.size();
    let error_size = consts_areas::ERROR_SIZE_AREA;
    let error_centered = dialog_render::dialog_centered(error_size, frame_area);

    let ok_area = consts_areas::ERROR_SIZE_OK;
    let ok_centered_area = dialog_render::ok_centered(ok_area, error_centered);
    ok_centered_area
}

pub fn error_hover(
    console_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    hover_event: MouseEvent,
) {
    let frame_area = console_frame.size();
    let error_size = consts_areas::ERROR_SIZE_AREA;
    let error_centered = dialog_render::dialog_centered(error_size, frame_area);

    let column = hover_event.column;
    let row = hover_event.row;

    if misc_ui::rect_point_in(column, row, error_centered) {
        the_app.hover_element = state_app::HOVER_ERROR_DIALOG.to_string();

        let area_ok = hover_error_ok_area(console_frame);
        if misc_ui::rect_point_in(column, row, area_ok) {
            the_app.hover_element = state_app::HOVER_ERROR_OK.to_string();
        }
    }
}
