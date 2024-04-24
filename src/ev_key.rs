#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::app_state;
use crate::the_types;

use crossterm::cursor::MoveTo;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::execute;
use std::io::stdout;

pub fn do_event_key(the_app: &mut app_state::DownApp, key_event: KeyEvent) {
    let the_code = key_event.code;
    if the_code == KeyCode::Tab {
        the_app.ui_state = the_types::UiState::State002NewPodcastName;
    }

    if the_app.ui_state == the_types::UiState::State001NewPodcastUrl {
        the_app.new_podcast_url = edit_text(&the_app.new_podcast_url, the_code);
    } else if the_app.ui_state == the_types::UiState::State002NewPodcastName {
        the_app.new_podcast_name = edit_text(&the_app.new_podcast_name, the_code);
    } else {
        if the_code == KeyCode::Up {
            the_app.scrolled_episodes_pos = the_app.scrolled_episodes_pos.saturating_sub(1);
        } else if the_code == KeyCode::Down {
            the_app.scrolled_episodes_pos = the_app.scrolled_episodes_pos.saturating_add(1);
        }
    }
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
    .expect("edit-text-err");
    ed_text2
}
