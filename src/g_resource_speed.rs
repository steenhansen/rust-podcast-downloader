#[allow(unused)]
use log::{debug, info, trace, warn};

use once_cell::sync::Lazy;
use std::sync::Mutex;

//pub static mut RESOURCE_SPEED: u16 = 0; // for easy updating from DownApp.fast_med_slow(0,1,2) in automonous threads

pub static RESOURCE_SPEED: Lazy<Mutex<u16>> = Lazy::new(|| {
    let fast_med_slow = 0;
    Mutex::new(fast_med_slow)
});

pub fn change_speed(new_speed: u16) {
    *RESOURCE_SPEED.lock().expect("resource-speed-change-err") = new_speed;
}
pub fn get_speed() -> u16 {
    *RESOURCE_SPEED.lock().expect("get-resource-speed-err")
}
