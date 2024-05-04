#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::components::buttons::*;
use crate::components::checkbox_pause;
use crate::components::episodes::episode_case;
use crate::components::input_address;
use crate::components::input_name;
use crate::components::podcasts::podcast_media;
use crate::components::radio_resource;
use crate::components::words_feed;
use crate::components::words_larger;
use crate::components::words_status;
use crate::components::words_title;
use crate::dialogs::dialog_error;
use crate::dialogs::dialog_help;
use crate::dialogs::dialog_sure;
use crate::globals::g_pause;
use crate::state::state_app;

use ratatui::prelude::*;

pub fn draw_ui(console_frame: &mut Frame, the_app: &mut state_app::DownApp) {
    let console_size = console_frame.size();
    if console_size.width < 95 || console_size.height < 13 {
        words_larger::title_show(console_frame);
    } else {
        let app_dim = state_app::app_dim(the_app);
        let is_downloading_paused = g_pause::pause_currently();

        words_title::title_show(console_frame, the_app, app_dim, is_downloading_paused);

        input_address::address_show(console_frame, the_app, app_dim, is_downloading_paused);

        input_name::name_show(console_frame, the_app, app_dim, is_downloading_paused);
        btn_new::new_show(console_frame, the_app, app_dim, is_downloading_paused);
        btn_every::every_show(console_frame, the_app);

        btn_stop::stop_show(console_frame, the_app);

        radio_resource::resources_show(console_frame, the_app, app_dim, is_downloading_paused);

        podcast_media::media_podcast_states(console_frame, the_app, app_dim, is_downloading_paused);
        episode_case::case_state_of_episodes(
            console_frame,
            the_app,
            app_dim,
            is_downloading_paused,
        );
        words_status::status_show(console_frame, the_app, app_dim, is_downloading_paused);

        words_feed::feed_show(console_frame, the_app, app_dim, is_downloading_paused);

        checkbox_pause::pause_show(console_frame, the_app);

        dialog_help::help_render(console_frame, the_app);

        dialog_error::error_render(console_frame, the_app);

        dialog_sure::sure_render(console_frame, the_app);

        btn_quit::quit_show(console_frame, the_app);
    }
}
