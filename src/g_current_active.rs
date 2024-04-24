#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::const_globals;
use crate::g_current_active;

use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

pub static G_CURRENT_ACTIVE: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});

pub fn new_pod_clear() {
    G_CURRENT_ACTIVE
        .lock()
        .expect("clear-active-new-podcast-err")
        .clear();
}

pub fn just_done(selected_podcast: String) -> usize {
    let number_just_done = G_CURRENT_ACTIVE
        .lock()
        .expect("current-done-count-err")
        .clone();
    let mut fini = 0;
    for (_file, num) in number_just_done {
        if _file.starts_with(&selected_podcast) {
            if num == const_globals::DOWNLOADED_MEDIA {
                fini += 1;
            }
        }
    }
    fini
}

pub fn active_downloading() -> usize {
    let number_downing = G_CURRENT_ACTIVE.lock().expect("active-count-err").clone();
    let mut actives = 0;
    for (_file, num) in number_downing {
        if num != const_globals::DOWNLOADED_MEDIA {
            actives += 1;
        }
    }
    actives
}

pub fn remove_status(local_file: &String) -> bool {
    G_CURRENT_ACTIVE.lock().expect("active-to-done-err").insert(
        local_file.clone(),
        const_globals::DOWNLOADED_MEDIA.to_string(),
    );
    let num_downloading = g_current_active::active_downloading();
    let finished_downloading = num_downloading == 0;
    finished_downloading
}

pub fn change_status(local_file: &String, byte_count: u32) {
    G_CURRENT_ACTIVE
        .lock()
        .expect("change-download-bytes-err")
        .insert(local_file.clone(), byte_count.to_string());
}

pub fn is_in(local_file: String) -> bool {
    let number_downing = G_CURRENT_ACTIVE
        .lock()
        .expect("episode-is-active-err")
        .clone();
    for (downloading_file, _num) in number_downing {
        if local_file == downloading_file {
            return true;
        }
    }
    false
}
