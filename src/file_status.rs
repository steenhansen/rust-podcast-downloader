use crate::const_globals;
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

//pub static mut RESOURCE_SPEED: u16 = 0; // for easy updating from DownApp.fast_med_slow(0,1,2) in automonous threads

pub static RESOURCE_SPEED: Lazy<Mutex<u16>> = Lazy::new(|| {
    let m = 0;

    Mutex::new(m)
});

pub fn change_speed(new_speed: u16) {
    *RESOURCE_SPEED.lock().unwrap() = new_speed;
}
pub fn get_speed() -> u16 {
    *RESOURCE_SPEED.lock().unwrap()
}
///////////////////
pub static G_SS: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let m = HashMap::new();

    Mutex::new(m)
});

pub fn get_gss() -> usize {
    let number_downing = G_SS.lock().unwrap().clone();
    let mut actives = 0;
    for (_file, num) in number_downing {
        if num != const_globals::DOWNLOADED_MEDIA {
            actives += 1;
        }
    }
    actives
}

pub fn remove_status(local_file: &String) {
    G_SS.lock().unwrap().insert(
        local_file.clone(),
        const_globals::DOWNLOADED_MEDIA.to_string(),
    );
    //        .insert(local_file.clone(), "-11".to_string());
    //  if num_bytes == const_globals::DOWNLOADED_MEDIA {
}

pub fn change_status(local_file: &String, byte_count: u32) {
    G_SS.lock()
        .unwrap()
        .insert(local_file.clone(), byte_count.to_string());
}
