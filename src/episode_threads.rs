//   https://rust-lang-nursery.github.io/rust-cookbook/web/clients/download.html?highlight=chunk#make-a-partial-download-with-http-range-headers

use crate::episodes_files;
use crate::file_log;
use crate::g_current_active;

#[allow(unused)]
use log::{debug, info, trace, warn};
use std::thread;

pub fn spawn_getter(sel_podcast: String, media_fname: String, url_episode: String) {
    let local_file = format!("{}/{}", sel_podcast, media_fname);
    if !g_current_active::is_in(local_file) {
        let _handle = thread::spawn(move || {
            file_log::trace_off();
            let _abc = episodes_files::read_file(sel_podcast, media_fname, url_episode);
            file_log::trace_on();
        });
    }
}
