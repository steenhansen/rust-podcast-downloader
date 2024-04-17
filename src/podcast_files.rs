//             //https://docs.rs/ratatui/latest/src/tabs/tabs.rs.html#144
use crate::const_globals;
use crate::down_app;
use crate::misc_fun;
use crate::rss_xml;

use crate::the_types;

use http_req::request;
use http_req::uri::Uri;
#[allow(unused)]
use log::{debug, info, trace, warn};
use std::collections::HashMap;
use std::fs;
use std::{convert::TryFrom, time::Duration};

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

pub fn get_epi_list(the_app: &mut down_app::DownApp) -> the_types::Result<()> {
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

    for title_and_url in &pos_titles_urls {
        let (pod_index, actual_title, actual_url) = title_and_url;
        let file_type = misc_fun::file_type_real(actual_url.to_string());
        let index_usized = *pod_index as usize;
        let str_index = format!(
            "{:0width$}",
            index_usized,
            width = const_globals::LEADING_0_SIZE
        );
        let title_str = str_index + "_" + actual_title.as_str() + "." + file_type.as_str();

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
