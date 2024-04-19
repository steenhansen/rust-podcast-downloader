#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::app_ui;

use crate::the_types;
use crossterm::cursor::MoveTo;
use crossterm::event::KeyCode;

use crossterm::event::KeyEvent;
use crossterm::execute;
use std::io::stdout;

pub fn do_event_key(the_app: &mut app_ui::DownApp, key_event: KeyEvent) -> () {
    let the_code = key_event.code;
    if the_code == KeyCode::Tab {
        the_app.ui_state = the_types::UiState::State001NewPodcastUrl;
    }
    match the_app.ui_state {
        the_types::UiState::State002NewPodcastName => {
            the_app.podcast_name = edit_text(&the_app.podcast_name, the_code);
        }
        the_types::UiState::State001NewPodcastUrl => {
            the_app.podcast_url = edit_text(&the_app.podcast_url, the_code);
        }
        _ => {
            match the_code {
                //KeyCode::Tab => the_app.ui_state = the_consts::UiState::State001NewPodcastUrl,
                //   KeyCode::Left => &the_app.decrement_counter(),
                //   KeyCode::Right => &the_app.increment_counter(),
                _ => (), //{}
            };
        }
    };
}

pub fn edit_text(ed_text: &String, the_code: KeyCode) -> String {
    let mut ed_text2 = ed_text.clone();
    match the_code {
        KeyCode::Char(c) => {
            ed_text2.push(c);
        }
        KeyCode::Backspace => {
            ed_text2.pop();
        }
        _ => {}
    }
    execute!(
        stdout(),
        MoveTo(13, 1),
        crossterm::cursor::SetCursorStyle::BlinkingUnderScore
    )
    .unwrap();
    ed_text2
}
