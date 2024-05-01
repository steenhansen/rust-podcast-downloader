#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::components::btn_every;
use crate::components::btn_new;
use crate::components::btn_quit;
use crate::components::btn_stop;
use crate::components::checkbox_pause;
use crate::episodes::episode_event;
use crate::state::app_state;

use crate::podcasts::podcast_event;

use crate::components::radio_resource;
use crate::events::ev_checks;

use crossterm::event::MouseEvent;
use ratatui::prelude::*;

pub fn do_click_mouse(
    the_frame: &mut Frame,
    the_app: &mut app_state::DownApp,
    mouse_event: MouseEvent,
) -> bool {
    btn_new::check_new(the_app, mouse_event);
    btn_every::check_every(the_app, mouse_event);
    podcast_event::check_podcasts(the_app, mouse_event, the_frame);
    episode_event::clicked_episodes(the_app, mouse_event, the_frame);
    radio_resource::check_resources(the_app, mouse_event);
    checkbox_pause::check_pause(the_app, mouse_event);
    btn_stop::check_stop(the_app, mouse_event);

    ev_checks::check_help(the_app, mouse_event, the_frame);
    ev_checks::check_error_ok(the_app, mouse_event, the_frame);

    ev_checks::check_sure_yes(the_app, mouse_event, the_frame);
    ev_checks::check_sure_no(the_app, mouse_event, the_frame);

    let is_quit = btn_quit::click_quit(mouse_event, the_frame);
    is_quit
}
