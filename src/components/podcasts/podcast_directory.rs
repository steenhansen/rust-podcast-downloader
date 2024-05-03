#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_globals;
use crate::state::state_app;

use http_req::request;
use http_req::uri::Uri;
use std::convert::TryFrom;
use std::error;
use std::fs;

pub fn directory_erase(the_app: &mut state_app::DownApp) {
    let rss_dir = format!("./{}", the_app.selected_podcast);
    if !the_app.init_erased_dirs.contains_key(&rss_dir) {
        let dir_entries = fs::read_dir(&rss_dir).expect("empty-dir-err");
        for an_entry in dir_entries {
            let the_entry = an_entry.expect("empty-dir-entry-err");
            let the_path = the_entry.path();
            if the_path.is_file() {
                let path_str = the_path.to_str().expect("path-to-str-err");
                if path_str.ends_with(const_globals::WORKING_FILE) {
                    fs::remove_file(path_str).expect("ends-with-err");
                }
            }
        }
        the_app.init_erased_dirs.insert(rss_dir, true);
    }
}

pub fn directory_get_filename(selected_podcast: String) -> String {
    let rss_file = format!("{}/{}", selected_podcast, const_globals::RSS_TEXT_FILE);
    let err_expect = "unable-2-read-file-err :: ".to_owned() + &rss_file;
    let rss_feed = fs::read_to_string(rss_file).expect(err_expect.as_str());
    rss_feed
}

pub fn directory_read_rss(podcast_url: &str) -> Result<String, Box<dyn error::Error>> {
    let mut writer = Vec::new();
    match Uri::try_from(podcast_url) {
        Ok(the_uri) => {
            let the_host = the_uri.host();
            match the_host {
                Some(have_host) => have_host,
                None => return Err("no-host".into()), // url == "www.xe.com"
            };
        }
        Err(e) => {
            return Err(Box::new(e)); // url == ""
        }
    };
    let uri_with_http = Uri::try_from(podcast_url).expect("podcast-rss-err");
    let mut request = request::Request::new(&uri_with_http);
    request.connect_timeout(const_globals::RSS_SOME_TIMEOUT);
    request.read_timeout(const_globals::RSS_SOME_TIMEOUT);
    match request.send(&mut writer) {
        Ok(the_response) => the_response,
        Err(e) => return Err(Box::new(e)), // url == "https://www.dont-exists.com"
    };
    let real_bytes = match std::str::from_utf8(&writer) {
        Ok(the_bytes) => the_bytes,
        Err(e) => return Err(Box::new(e)),
    };
    let real_str = real_bytes.to_string(); // url == "https://www.xe.com"
    Ok(real_str)
}
