#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::consts_globals;
use crate::consts::consts_types;
use crate::state::state_app;

use crossterm::cursor::MoveTo;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::execute;
use std::io::stdout;

pub fn key_scan(the_app: &mut state_app::DownApp, key_event: KeyEvent) {
    let the_code = key_event.code;
    if the_code == KeyCode::Tab {
        the_app.ui_state = consts_types::UiState::State002NewPodcastName;
    } else if the_code == KeyCode::PageUp {
        the_app.scrolled_episodes_pos = the_app
            .scrolled_episodes_pos
            .saturating_sub(consts_globals::PAGE_SIZE);
    } else if the_code == KeyCode::PageDown {
        let num_episodes = the_app.ordered_episodes.len();
        let epi_scroll_pos = the_app.scrolled_episodes_pos;
        if epi_scroll_pos + consts_globals::PAGE_SIZE < num_episodes {
            the_app.scrolled_episodes_pos = the_app
                .scrolled_episodes_pos
                .saturating_add(consts_globals::PAGE_SIZE);
        }
        return;
    } else if the_code == KeyCode::Up {
        the_app.scrolled_episodes_pos = the_app.scrolled_episodes_pos.saturating_sub(1);
    } else if the_code == KeyCode::Down {
        let num_episodes = the_app.ordered_episodes.len();
        let epi_scroll_pos = the_app.scrolled_episodes_pos;
        if epi_scroll_pos + 1 < num_episodes {
            the_app.scrolled_episodes_pos = the_app.scrolled_episodes_pos.saturating_add(1);
        }
    } else if the_code == KeyCode::Char('H') || the_code == KeyCode::Char('h') {
        the_app.pause_help = the_app.ui_state;
        the_app.ui_state = consts_types::UiState::State501Help;
    } else if the_app.ui_state == consts_types::UiState::State001NewPodcastUrl {
        let long_url = key_edit_text(&the_app.new_podcast_url, the_code);
        let trimmed_url = long_url.trim();
        the_app.new_podcast_url = trimmed_url.to_string();
    } else if the_app.ui_state == consts_types::UiState::State002NewPodcastName {
        let long_name = key_edit_text(&the_app.new_podcast_name, the_code);
        let trimmed_name = long_name.trim();
        the_app.new_podcast_name = trimmed_name.to_string();
    }
}

pub fn key_edit_text(ed_text: &String, the_code: KeyCode) -> String {
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
