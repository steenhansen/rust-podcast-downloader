#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::events::ev_click_ui;
use crate::events::ev_hover;
use crate::events::ev_key;
use crate::events::ev_scroll;
use crate::misc::misc_ui;
use crate::state::state_app;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::prelude::*;

pub fn all_events(the_frame: &mut Frame, the_app: &mut state_app::DownApp) -> bool {
    match event::read().expect("event-key-err") {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            if misc_ui::is_control_c(key_event) {
                return true;
            }
            ev_key::key_scan(the_app, key_event)
        }
        Event::Mouse(mouse_event) if misc_ui::mouse_click(mouse_event) => {
            let finished = ev_click_ui::click_ui_mouse(the_frame, the_app, mouse_event);
            return finished;
        }
        Event::Mouse(mouse_event) if misc_ui::mouse_scroll(mouse_event) => {
            ev_scroll::scroll_lists(the_app, mouse_event, the_frame);
        }
        Event::Mouse(mouse_event) => {
            ev_hover::hover_ui(the_frame, the_app, mouse_event);
        }

        _ => {}
    };
    false
}
