use crate::const_globals;
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};
pub static G_SS: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let m = HashMap::new();

    Mutex::new(m)
});

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
