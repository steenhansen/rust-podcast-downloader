#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::components::btn_every;
use crate::components::btn_new;
use crate::components::btn_quit;
use crate::components::btn_stop;
use crate::components::checkbox_pause;
use crate::components::input_address;
use crate::components::input_name;
use crate::consts::area_rects;
use crate::state::app_state;

use crate::components::radio_resource;
use crate::episodes::episode_event;

use crate::podcasts::podcast_event;

use crossterm::event::MouseEvent;
use ratatui::prelude::*;

pub fn do_hovers(the_frame: &mut Frame, the_app: &mut app_state::DownApp, hover_event: MouseEvent) {
    let column = hover_event.column;
    let row = hover_event.row;

    the_app.hover_element = app_state::HOVER_NONE.to_string();

    btn_new::new_hover(the_app, hover_event);
    btn_every::every_hover(the_app, hover_event);
    btn_stop::stop_hover(the_app, hover_event);
    btn_quit::quit_hover(the_frame, the_app, hover_event);
    checkbox_pause::pause_hover(the_app, hover_event);
    radio_resource::resources_hover(the_app, hover_event);
    input_address::address_hover(the_app, hover_event);
    input_name::name_hover(the_app, hover_event);
    episode_event::episode_hover(the_frame, the_app, hover_event);
    podcast_event::podcast_hover(the_frame, the_app, hover_event);

    let dialog_ok_area = area_rects::ok_dialog_area(the_frame);
    if area_rects::point_in_rect(column, row, dialog_ok_area) {
        the_app.hover_element = app_state::HOVER_OK_DIALOG.to_string();
        the_app.podcast_hover_row = (row - dialog_ok_area.y) as usize;
    }

    let yes_sure_area = area_rects::yes_are_sure_dialog_area(the_frame);
    if area_rects::point_in_rect(column, row, yes_sure_area) {
        the_app.hover_element = app_state::HOVER_YES_SURE.to_string();
    }

    let no_sure_area = area_rects::no_are_sure_dialog_area(the_frame);
    if area_rects::point_in_rect(column, row, no_sure_area) {
        the_app.hover_element = app_state::HOVER_NO_SURE.to_string();
    }
}
