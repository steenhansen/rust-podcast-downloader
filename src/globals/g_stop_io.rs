#[allow(unused)]
use log::{debug, info, trace, warn};

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static KILLING_DOWNLOADS: Lazy<Mutex<bool>> = Lazy::new(|| {
    let is_stoping = false;
    Mutex::new(is_stoping)
});

pub fn is_stoping() -> bool {
    let is_stoping = *KILLING_DOWNLOADS.lock().expect("current-pause-err");
    is_stoping
}

pub fn start_stoping() {
    *KILLING_DOWNLOADS.lock().expect("current-pause-set-err") = true;
}

pub fn stop_stoping() {
    *KILLING_DOWNLOADS.lock().expect("current-pause-set-err") = false;
}
