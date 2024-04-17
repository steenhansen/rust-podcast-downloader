#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::ev_click;
use crate::ev_key;

use crate::down_app;
use crate::ev_scroll;
use crate::misc_fun;

use crossterm::event::{self, Event, KeyEventKind};

use ratatui::prelude::*;

pub fn all_events_done(the_frame: &mut Frame, the_app: &mut down_app::DownApp) -> bool {
    match event::read().unwrap() {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            if misc_fun::is_control_c(key_event) {
                return true;
            }
            ev_key::do_event_key(the_app, key_event)
        }
        Event::Mouse(mouse_event) if misc_fun::mouse_click(mouse_event) => {
            let finished = ev_click::do_click_mouse(the_frame, the_app, mouse_event);
            return finished;
        }
        Event::Mouse(mouse_event) if misc_fun::mouse_scroll(mouse_event) => {
            ev_scroll::do_pod_scroll(the_app, mouse_event)
        }
        _ => {}
    };
    false
}
