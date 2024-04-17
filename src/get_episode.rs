//   https://rust-lang-nursery.github.io/rust-cookbook/web/clients/download.html?highlight=chunk#make-a-partial-download-with-http-range-headers

use crate::episodes_files;
use crate::file_log;

#[allow(unused)]
use log::{debug, info, trace, warn};
use std::thread;

pub fn spawn_getter(sel_podcast: String, file_name: String, url_episode: String) {
    let _handle = thread::spawn(move || {
        file_log::trace_off();
        let _abc = episodes_files::read_file(sel_podcast, file_name, url_episode);

        file_log::trace_on();
    });
}
