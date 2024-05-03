#[allow(unused)]
use log::{debug, info, trace, warn};

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static PAUSE_PROGRAM: Lazy<Mutex<bool>> = Lazy::new(|| {
    let is_paused = false;
    Mutex::new(is_paused)
});

pub fn pause_currently() -> bool {
    let is_paused = *PAUSE_PROGRAM.lock().expect("current-pause-err");
    is_paused
}

pub fn pause_flip() -> bool {
    let current_pausing = *PAUSE_PROGRAM.lock().expect("current-pause-flip-err");
    let new_pausing = !current_pausing;
    *PAUSE_PROGRAM.lock().expect("current-pause-set-err") = new_pausing;
    new_pausing
}
