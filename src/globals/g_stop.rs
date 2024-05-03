#[allow(unused)]
use log::{debug, info, trace, warn};

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static STOP_DOWNLOADS: Lazy<Mutex<bool>> = Lazy::new(|| {
    let stop_current = false;
    Mutex::new(stop_current)
});

pub fn stop_current() -> bool {
    let stop_current = *STOP_DOWNLOADS.lock().expect("current-pause-err");
    stop_current
}

pub fn stop_true() {
    *STOP_DOWNLOADS.lock().expect("current-pause-set-err") = true;
}

pub fn stop_false() {
    *STOP_DOWNLOADS.lock().expect("current-pause-set-err") = false;
}
