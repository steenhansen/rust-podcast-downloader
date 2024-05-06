#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_areas;
use crate::consts::const_globals;
use crate::consts::const_types;
use crate::state::state_app;

use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

pub fn key_scan(the_app: &mut state_app::DownApp, key_event: KeyEvent) {
    let the_code = key_event.code;
    if the_code == KeyCode::Tab && the_app.ui_state == const_types::UiState::State001NewPodcastUrl {
        the_app.ui_state = const_types::UiState::State002NewPodcastName;
    } else if the_code == KeyCode::Tab
        && the_app.ui_state == const_types::UiState::State002NewPodcastName
    {
        the_app.ui_state = const_types::UiState::State001NewPodcastUrl;
    } else if the_code == KeyCode::PageUp {
        the_app.scrolled_episodes_pos = the_app
            .scrolled_episodes_pos
            .saturating_sub(const_globals::PAGE_SIZE);
    } else if the_code == KeyCode::PageDown {
        let num_episodes = the_app.ordered_episodes.len();
        let epi_scroll_pos = the_app.scrolled_episodes_pos;
        if epi_scroll_pos + const_globals::PAGE_SIZE < num_episodes {
            the_app.scrolled_episodes_pos = the_app
                .scrolled_episodes_pos
                .saturating_add(const_globals::PAGE_SIZE);
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
    } else if the_app.ui_state == const_types::UiState::State001NewPodcastUrl {
        let url_max_len = const_areas::MAX_URL_WIDTH;
        let long_url = key_edit_text(&the_app.new_podcast_url, the_code, url_max_len);
        let trimmed_url = long_url.trim();
        the_app.new_podcast_url = trimmed_url.to_string();
    } else if the_app.ui_state == const_types::UiState::State002NewPodcastName {
        let name_max_len = const_areas::MAX_NAME_WIDTH;
        let long_name = key_edit_text(&the_app.new_podcast_name, the_code, name_max_len);
        the_app.new_podcast_name = long_name;
    } else if the_code == KeyCode::Char('H') || the_code == KeyCode::Char('h') {
        // NB must be last so can type h into text boxes
        the_app.pause_help = the_app.ui_state;
        the_app.ui_state = const_types::UiState::State501Help;
    }
}

pub fn key_edit_text(ed_text: &String, the_code: KeyCode, max_len: u16) -> String {
    let max_len_usize = max_len.into();
    let mut ed_text2 = ed_text.clone();
    match the_code {
        KeyCode::Char(c) => {
            if ed_text2.len() < max_len_usize {
                ed_text2.push(c);
            }
        }
        KeyCode::Backspace => {
            ed_text2.pop();
        }
        _ => {}
    }
    ed_text2
}
