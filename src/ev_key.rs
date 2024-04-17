#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::input_box;
use crate::render_app;
use crate::the_types;

use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

pub fn do_event_key(the_app: &mut render_app::DownApp, key_event: KeyEvent) -> () {
    let the_code = key_event.code;
    if the_code == KeyCode::Tab {
        the_app.ui_state = the_types::UiState::State001NewPodcastUrl;
    }
    match the_app.ui_state {
        the_types::UiState::State002NewPodcastName => {
            the_app.podcast_name = input_box::edit_text(&the_app.podcast_name, the_code);
        }
        the_types::UiState::State001NewPodcastUrl => {
            the_app.podcast_url = input_box::edit_text(&the_app.podcast_url, the_code);
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
