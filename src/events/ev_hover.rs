#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::components::buttons::*;
use crate::components::checkbox_pause;
use crate::components::episodes::episode_acts;
use crate::components::input_address;
use crate::components::input_name;
use crate::components::podcasts::podcast_happenings;
use crate::components::radio_resource;
use crate::consts::const_types;
use crate::dialogs::*;
use crate::state::state_app;

use crossterm::event::MouseEvent;
use ratatui::prelude::*;

pub fn hover_ui(the_frame: &mut Frame, the_app: &mut state_app::DownApp, hover_event: MouseEvent) {
    the_app.hover_element = state_app::HOVER_NONE.to_string();

    btn_new::new_hover(the_app, hover_event);
    btn_every::every_hover(the_app, hover_event);
    btn_stop::stop_hover(the_app, hover_event);
    btn_quit::quit_hover(the_frame, the_app, hover_event);
    checkbox_pause::pause_hover(the_app, hover_event);
    radio_resource::resources_hover(the_app, hover_event);
    input_address::address_hover(the_app, hover_event);
    input_name::name_hover(the_app, hover_event);

    if the_app.ui_state == const_types::UiState::State103ShowEpisodes {
        episode_acts::acts_episode_hover(the_frame, the_app, hover_event);
    }

    podcast_happenings::happening_podcast_hover(the_frame, the_app, hover_event);

    if the_app.ui_state == const_types::UiState::State501Help {
        dialog_help::help_hover(the_frame, the_app, hover_event);
    }

    if the_app.ui_state == const_types::UiState::State301WaitForPopErrorClose {
        dialog_error::error_hover(the_frame, the_app, hover_event);
    }

    if the_app.ui_state == const_types::UiState::State201EveryEpisode {
        dialog_sure::sure_hover(the_frame, the_app, hover_event);
    }
}
