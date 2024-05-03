#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::components::buttons::*;
use crate::components::checkbox_pause;
use crate::components::episodes::episode_acts;
use crate::components::podcasts::podcast_happenings;
use crate::components::radio_resource;
use crate::events::ev_clicked_filter;
use crate::state::state_app;

use crossterm::event::MouseEvent;
use ratatui::prelude::*;

pub fn click_ui_mouse(
    the_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    mouse_event: MouseEvent,
) -> bool {
    btn_new::new_clicked(the_app, mouse_event);
    btn_every::every_clicked(the_app, mouse_event);
    podcast_happenings::happening_clicked_podcasts(the_app, mouse_event, the_frame);
    episode_acts::acts_clicked_episodes(the_app, mouse_event, the_frame);
    radio_resource::resources_clicked(the_app, mouse_event);
    checkbox_pause::pause_clicked(the_app, mouse_event);
    btn_stop::stop_clicked(the_app, mouse_event);

    ev_clicked_filter::clicked_help(the_app, mouse_event, the_frame);
    ev_clicked_filter::clicked_error_ok(the_app, mouse_event, the_frame);

    ev_clicked_filter::clicked_sure_yes(the_app, mouse_event, the_frame);
    ev_clicked_filter::clicked_sure_no(the_app, mouse_event, the_frame);

    let is_quit = btn_quit::quit_clicked(mouse_event, the_frame);
    is_quit
}
