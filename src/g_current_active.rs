#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::const_globals;
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

pub static G_CURRENT_ACTIVE: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let m = HashMap::new();

    Mutex::new(m)
});

pub fn active_downloading() -> usize {
    let number_downing = G_CURRENT_ACTIVE.lock().unwrap().clone();
    let mut actives = 0;
    for (_file, num) in number_downing {
        if num != const_globals::DOWNLOADED_MEDIA {
            actives += 1;
        }
    }
    actives
}

pub fn remove_status(local_file: &String) {
    G_CURRENT_ACTIVE.lock().unwrap().insert(
        local_file.clone(),
        const_globals::DOWNLOADED_MEDIA.to_string(),
    );
}

pub fn change_status(local_file: &String, byte_count: u32) {
    G_CURRENT_ACTIVE
        .lock()
        .unwrap()
        .insert(local_file.clone(), byte_count.to_string());
}

pub fn is_in(local_file: String) -> bool {
    let number_downing = G_CURRENT_ACTIVE.lock().unwrap().clone();
    for (downloading_file, _num) in number_downing {
        if local_file == downloading_file {
            return true;
        }
    }
    false
}
