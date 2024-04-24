#[allow(unused)]
use log::{debug, info, trace, warn};

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static PROGRAM_PAUSED: Lazy<Mutex<bool>> = Lazy::new(|| {
    let is_paused = false;
    Mutex::new(is_paused)
});

pub fn is_cur_paused() -> bool {
    let is_paused = *PROGRAM_PAUSED.lock().expect("current-pause-err");
    is_paused
}

pub fn pause_flip() -> bool {
    let current_pausing = *PROGRAM_PAUSED.lock().expect("current-pause-flip-err");
    let new_pausing = !current_pausing;
    *PROGRAM_PAUSED.lock().expect("current-pause-set-err") = new_pausing;
    new_pausing
}
