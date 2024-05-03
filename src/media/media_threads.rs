#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_globals;
use crate::globals::g_active;
use crate::globals::g_pause;
use crate::globals::g_speed;
use crate::media::media_chunks;
use crate::state::state_app;

use std::thread;
use std::time::Duration;

pub fn threads_queue(
    the_app: &mut state_app::DownApp,
    sel_podcast: String,
    media_fname: String,
    url_episode: String,
) {
    let local_file = format!("{}/{}", sel_podcast, media_fname);
    if !g_active::active_is_in(local_file.clone()) {
        the_app
            .download_deque
            .push_back((sel_podcast, media_fname, url_episode));
    }
}

fn threads_spawn(sel_podcast: String, media_fname: String, url_episode: String) {
    let local_file = format!("{}/{}", sel_podcast, media_fname);

    if !g_active::active_is_in(local_file) {
        let _handle = thread::spawn(move || {
            let _abc = media_chunks::chunks_read(sel_podcast, media_fname, url_episode);
        });
    }
}

pub fn threads_limit(the_app: &mut state_app::DownApp) {
    let active_downs = g_active::active_downloading();
    let cur_speed = g_speed::speed_get();
    let max_cur_downs: usize = match cur_speed {
        0 => const_globals::MAX_SPAWNS_FAST,
        1 => const_globals::MAX_SPAWNS_MED,
        _ => const_globals::MAX_SPAWNS_SLOW,
    };
    if active_downs < max_cur_downs {
        if the_app.download_deque.len() > 0 {
            match the_app.download_deque.pop_front() {
                Some((sel_podcast, media_fname, url_episode)) => {
                    threads_spawn(sel_podcast, media_fname, url_episode)
                }
                None => (),
            };
        }
    }
}

pub fn chunks_sleep() -> bool {
    let cur_speed = g_speed::speed_get();
    match cur_speed {
        0 => (),
        1 => thread::sleep(Duration::from_secs(const_globals::SLEEP_SEC_MED)),
        _ => thread::sleep(Duration::from_secs(const_globals::SLEEP_SEC_SLOW)),
    }
    let currently_paused = g_pause::pause_currently();
    if currently_paused {
        thread::sleep(Duration::from_secs(const_globals::PAUSE_SLEEP_SEC));
    }
    currently_paused
}
