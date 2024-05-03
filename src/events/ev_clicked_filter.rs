#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_types;
use crate::dialogs::dialog_error;
use crate::dialogs::dialog_help;
use crate::dialogs::dialog_sure;
use crate::misc::misc_ui;
use crate::state::state_app;

use crossterm::event::MouseEvent;
use ratatui::prelude::*;

pub fn clicked_help(
    the_app: &mut state_app::DownApp,
    the_click: MouseEvent,
    the_frame: &mut Frame,
) -> () {
    if the_app.ui_state == const_types::UiState::State501Help {
        let column = the_click.column;
        let row = the_click.row;
        let error_close_area = dialog_help::hover_help_ok_area(the_frame);
        if misc_ui::rect_point_in(column, row, error_close_area) {
            the_app.ui_state = the_app.pause_help;
        }
    }
}

pub fn clicked_error_ok(
    the_app: &mut state_app::DownApp,
    the_click: MouseEvent,
    the_frame: &mut Frame,
) -> () {
    if the_app.ui_state == const_types::UiState::State301WaitForPopErrorClose {
        let column = the_click.column;
        let row = the_click.row;
        let error_close_area = dialog_error::hover_error_ok_area(the_frame);
        if misc_ui::rect_point_in(column, row, error_close_area) {
            the_app.ui_state = const_types::UiState::StateNoFocus;
        }
    }
}

pub fn clicked_sure_yes(
    the_app: &mut state_app::DownApp,
    the_click: MouseEvent,
    the_frame: &mut Frame,
) -> () {
    if the_app.ui_state == const_types::UiState::State201EveryEpisode {
        let column = the_click.column;
        let row = the_click.row;
        let error_close_area = dialog_sure::hover_sure_yes_area(the_frame);
        if misc_ui::rect_point_in(column, row, error_close_area) {
            the_app.ui_state = const_types::UiState::State202SureEveryEpisode;
        }
    }
}

pub fn clicked_sure_no(
    the_app: &mut state_app::DownApp,
    the_click: MouseEvent,
    the_frame: &mut Frame,
) -> () {
    if the_app.ui_state == const_types::UiState::State201EveryEpisode {
        let column = the_click.column;
        let row = the_click.row;
        let error_close_area = dialog_sure::hover_sure_no_area(the_frame);
        if misc_ui::rect_point_in(column, row, error_close_area) {
            the_app.ui_state = const_types::UiState::State103ShowEpisodes;
        }
    }
}
