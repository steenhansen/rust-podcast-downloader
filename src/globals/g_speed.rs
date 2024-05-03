#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_globals;

use once_cell::sync::Lazy;
use std::sync::Mutex;

//pub static mut SPEED_RESOURCE: u16 = 0; // for easy updating from DownApp.fast_med_slow(0,1,2) in automonous threads

pub static SPEED_RESOURCE: Lazy<Mutex<u16>> = Lazy::new(|| {
    let fast_med_slow = const_globals::RESOURCE_FAST;
    Mutex::new(fast_med_slow)
});

pub fn speed_change(new_speed: u16) {
    *SPEED_RESOURCE.lock().expect("resource-speed-change-err") = new_speed;
}
pub fn speed_get() -> u16 {
    *SPEED_RESOURCE.lock().expect("get-resource-speed-err")
}
