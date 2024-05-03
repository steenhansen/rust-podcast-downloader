#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::consts_globals;

use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

static ACTIVE_CURRENT: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});

pub fn active_clear() {
    ACTIVE_CURRENT
        .lock()
        .expect("clear-active-new-podcast-err")
        .clear();
}

pub fn active_just_done(selected_podcast: String) -> usize {
    let number_just_done = ACTIVE_CURRENT
        .lock()
        .expect("current-done-count-err")
        .clone();
    let mut fini = 0;
    for (_file, num) in number_just_done {
        if _file.starts_with(&selected_podcast) {
            if num == consts_globals::DOWN_BYTES_FINISHED {
                fini += 1;
            }
        }
    }
    fini
}

pub fn active_downloading() -> usize {
    let number_downing = ACTIVE_CURRENT.lock().expect("active-count-err").clone();
    let mut actives = 0;
    for (_file, num) in number_downing {
        if num != consts_globals::DOWN_BYTES_FINISHED
            && num != consts_globals::DOWN_BYTES_STOPPED
            && num != consts_globals::DOWN_BYTES_INITL_SHOW
        {
            actives += 1;
        }
    }
    actives
}

pub fn active_stopped(local_file: &String) {
    ACTIVE_CURRENT
        .lock()
        .expect("change-download-bytes-err")
        .insert(
            local_file.clone(),
            consts_globals::DOWN_BYTES_STOPPED.to_string(),
        );
}

pub fn active_remove(local_file: &String) -> bool {
    ACTIVE_CURRENT.lock().expect("active-to-done-err").insert(
        local_file.clone(),
        consts_globals::DOWN_BYTES_FINISHED.to_string(),
    );
    let num_downloading = active_downloading();
    let finished_downloading = num_downloading == 0;
    finished_downloading
}

pub fn active_change(local_file: &String, byte_count: u32) {
    if byte_count == 0 {
        ACTIVE_CURRENT
            .lock()
            .expect("change-download-bytes-err")
            .insert(
                local_file.clone(),
                consts_globals::DOWN_BYTES_INITL_SHOW.to_string(),
            );
    } else if byte_count == 1 {
        ACTIVE_CURRENT
            .lock()
            .expect("change-download-bytes-err")
            .insert(
                local_file.clone(),
                consts_globals::DOWN_BYTES_START_SHOW.to_string(),
            );
    } else {
        ACTIVE_CURRENT
            .lock()
            .expect("change-download-bytes-err")
            .insert(local_file.clone(), byte_count.to_string());
    }
}

pub fn active_is_in(local_file: String) -> bool {
    let number_downing = ACTIVE_CURRENT
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

pub fn active_bytes(local_file: String) -> String {
    let number_downing = ACTIVE_CURRENT
        .lock()
        .expect("episode-is-active-err")
        .clone();
    for (downloading_file, str_num) in number_downing {
        if local_file == downloading_file {
            if str_num == consts_globals::DOWN_BYTES_STOPPED {
                return "".to_string();
            }
            return str_num;
        }
    }
    "".to_string()
}
