#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::area_rects;
use crate::consts::the_types;
use crate::state::app_state;

use crossterm::event::MouseEvent;
use ratatui::prelude::*;

pub fn check_help(
    the_app: &mut app_state::DownApp,
    the_click: MouseEvent,
    the_frame: &mut Frame,
) -> () {
    if the_app.ui_state == the_types::UiState::State501Help {
        let column = the_click.column;
        let row = the_click.row;
        let error_close_area = area_rects::ok_dialog_area(the_frame);
        if area_rects::point_in_rect(column, row, error_close_area) {
            the_app.ui_state = the_app.pause_help;
            //the_app.ui_state = the_types::UiState::StateNoFocus;
        }
    }
}

pub fn check_error_ok(
    the_app: &mut app_state::DownApp,
    the_click: MouseEvent,
    the_frame: &mut Frame,
) -> () {
    if the_app.ui_state == the_types::UiState::State301WaitForPopErrorClose {
        let column = the_click.column;
        let row = the_click.row;
        let error_close_area = area_rects::ok_dialog_area(the_frame);
        if area_rects::point_in_rect(column, row, error_close_area) {
            the_app.ui_state = the_types::UiState::StateNoFocus;
        }
    }
}

pub fn check_sure_yes(
    the_app: &mut app_state::DownApp,
    the_click: MouseEvent,
    the_frame: &mut Frame,
) -> () {
    if the_app.ui_state == the_types::UiState::State201EveryEpisode {
        let column = the_click.column;
        let row = the_click.row;
        let error_close_area = area_rects::yes_are_sure_dialog_area(the_frame);
        if area_rects::point_in_rect(column, row, error_close_area) {
            the_app.ui_state = the_types::UiState::State202SureEveryEpisode;
        }
    }
}

pub fn check_sure_no(
    the_app: &mut app_state::DownApp,
    the_click: MouseEvent,
    the_frame: &mut Frame,
) -> () {
    if the_app.ui_state == the_types::UiState::State201EveryEpisode {
        let column = the_click.column;
        let row = the_click.row;
        let error_close_area = area_rects::no_are_sure_dialog_area(the_frame);
        if area_rects::point_in_rect(column, row, error_close_area) {
            the_app.ui_state = the_types::UiState::State103ShowEpisodes;
        }
    }
}
