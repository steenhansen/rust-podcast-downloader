//             //https://docs.rs/ratatui/latest/src/tabs/tabs.rs.html#144
use crate::app_ui;
use crate::const_globals;
use crate::misc_fun;
use crate::podcast_scroll;
use crate::rss_xml;

use crate::the_types;

use http_req::request;
use http_req::uri::Uri;
#[allow(unused)]
use log::{debug, info, trace, warn};
use std::collections::HashMap;
use std::fs;
use std::{convert::TryFrom, time::Duration};

pub fn erase_working(the_app: &mut app_ui::DownApp) {
    // std::io::Result<()> {
    let rss_dir = format!("./{}", the_app.selected_podcast);
    if !the_app.init_erased_dirs.contains_key(&rss_dir) {
        let dir_entries = fs::read_dir(&rss_dir).unwrap();
        for an_entry in dir_entries {
            let the_entry = an_entry.expect("bard2");
            let the_path = the_entry.path();
            if the_path.is_file() {
                let path_str = the_path.to_str().unwrap();
                if path_str.ends_with(const_globals::WORKING_FILE) {
                    fs::remove_file(path_str).unwrap(); //?;
                }
            }
        }
        the_app.init_erased_dirs.insert(rss_dir, true);
    }
}

pub fn read_rss22(file_name: &str) -> the_types::Result<String> {
    let mut writer = Vec::new();
    let uri = Uri::try_from(file_name).unwrap();
    let mut request = request::Request::new(&uri);
    request.connect_timeout(Some(Duration::from_secs(3600)));
    request.read_timeout(Some(Duration::from_secs(3600)));
    let _reponse_x = match request.send(&mut writer) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };
    let real_bytes = match std::str::from_utf8(&writer) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };
    let real_str = real_bytes.to_string();
    Ok(real_str)
}

pub fn get_epi_list(the_app: &mut app_ui::DownApp) -> the_types::Result<()> {
    erase_working(the_app);
    let rss_file = format!(
        "{}/{}",
        the_app.selected_podcast,
        const_globals::RSS_TEXT_FILE
    );
    let rss_feed = fs::read_to_string(rss_file).expect("Unable to read file");
    let an_string = match read_rss22(&rss_feed) {
        Ok(an_string) => an_string,
        Err(e) => {
            return Err(e);
        }
    };
    let neg_titles_urls = match rss_xml::negative_titles_urls(an_string) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let pos_titles_urls = rss_xml::positive_titles_urls(neg_titles_urls);
    the_app.ordered_episodes = Vec::new();
    the_app.episode_2_url = HashMap::new();

    let is_in_prefix =
        podcast_scroll::is_int_prefix(const_globals::ROOT_DIR, &the_app.selected_podcast);

    for title_and_url in &pos_titles_urls {
        let (pod_index, actual_title, actual_url) = title_and_url;
        let file_type = misc_fun::file_type_real(actual_url.to_string());
        let episode_index = *pod_index as usize;

        let mut title_str = actual_title.to_owned() + "." + file_type.as_str();

        if is_in_prefix {
            let str_index = misc_fun::epi_prefix_num(episode_index);
            title_str = str_index.to_owned() + "_" + &title_str;
        }

        the_app.ordered_episodes.push(title_str.clone());
        the_app
            .episode_2_url
            .insert(title_str.clone(), actual_url.clone());
    }
    the_app.scrolled_episodes = 0;
    the_app.state_scroll_episodes = the_app
        .state_scroll_episodes
        .content_length(the_app.ordered_episodes.len());

    Ok(())
}
