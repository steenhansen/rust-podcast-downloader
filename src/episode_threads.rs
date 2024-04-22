//   https://rust-lang-nursery.github.io/rust-cookbook/web/clients/download.html?highlight=chunk#make-a-partial-download-with-http-range-headers

use crate::app_state;
use crate::const_globals;
use crate::episodes_files;
use crate::g_current_active;
use crate::g_resource_speed;
#[allow(unused)]
use log::{debug, info, trace, warn};
use std::thread;

pub fn queue_episode_download(
    the_app: &mut app_state::DownApp,
    sel_podcast: String,
    media_fname: String,
    url_episode: String,
) {
    let local_file = format!("{}/{}", sel_podcast, media_fname);
    if !g_current_active::is_in(local_file.clone()) {
        warn!(" quei {:?}", local_file);
        // g_current_active::change_status(&local_file, 0);
        the_app
            .download_deque
            .push_back((sel_podcast, media_fname, url_episode));
    }
}

fn spawn_it(sel_podcast: String, media_fname: String, url_episode: String) {
    let local_file = format!("{}/{}", sel_podcast, media_fname);

    if !g_current_active::is_in(local_file) {
        let _handle = thread::spawn(move || {
            let _abc = episodes_files::read_file(sel_podcast, media_fname, url_episode);
        });
    }
}

pub fn check_start_down(the_app: &mut app_state::DownApp) {
    //warn!("check_start_dwon");
    let active_downs = g_current_active::active_downloading();
    let cur_speed = g_resource_speed::get_speed();
    let max_cur_downs: usize = match cur_speed {
        0 => const_globals::MAX_SPAWNS_FAST,
        1 => const_globals::MAX_SPAWNS_MED,
        _ => const_globals::MAX_SPAWNS_SLOW,
    };
    if active_downs < max_cur_downs {
        if the_app.download_deque.len() > 0 {
            match the_app.download_deque.pop_front() {
                Some((sel_podcast, media_fname, url_episode)) => {
                    spawn_it(sel_podcast, media_fname, url_episode)
                }
                None => (),
            };
        }
    }
}
