#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::the_types::*;
use crate::files::rss_xml;
use crate::misc::misc_fun;
use crate::podcasts::podcast_files;
use crate::state::app_state;

use std::collections::HashMap;
use std::error;

pub fn get_epi_list(the_app: &mut app_state::DownApp) -> Result<(), Box<dyn error::Error>> {
    podcast_files::erase_working(the_app);
    let rss_feed = podcast_files::get_filename(the_app.selected_podcast.clone());
    let an_string = match podcast_files::read_rss(&rss_feed) {
        Ok(an_string) => an_string,
        Err(_e) => {
            let bad_url_err = format!("\n Error - {:?} is not valid URL", &rss_feed);
            return Err(bad_url_err.into());
        }
    };
    let neg_titles_urls = match rss_xml::dirty_titles_urls(an_string) {
        Ok(v) => v,
        Err(_e) => {
            let bad_xml_err = format!("\n Error - {:?} is not valid XML", &rss_feed);
            return Err(bad_xml_err.into());
        }
    };
    clear_podcast_data(the_app);
    slurp_data(the_app, neg_titles_urls);
    the_app.scrolled_episodes_pos = 0;
    the_app.state_scroll_episodes = the_app
        .state_scroll_episodes
        .content_length(the_app.ordered_episodes.len());

    Ok(())
}

fn slurp_data(the_app: &mut app_state::DownApp, neg_titles_urls: Vec<EpisodeMetadataTuple>) {
    let pos_titles_urls = rss_xml::clean_titles_urls(neg_titles_urls);
    for title_and_url in &pos_titles_urls {
        let (pod_index, actual_title, actual_url, actual_len) = title_and_url;
        let mut file_dot_type = file_with_type(actual_title, actual_url);
        file_dot_type = add_working(file_dot_type, pod_index);
        the_app.ordered_episodes.push(file_dot_type.clone());
        the_app
            .episode_2_url
            .insert(file_dot_type.clone(), actual_url.clone());

        the_app
            .episode_2_len
            .insert(file_dot_type.clone(), *actual_len);
    }
}

fn clear_podcast_data(the_app: &mut app_state::DownApp) {
    the_app.ordered_episodes = Vec::new();
    the_app.episode_2_url = HashMap::new();
    the_app.episode_2_len = HashMap::new();
}

fn add_working(file_dot_type: String, pod_index: &i32) -> String {
    let episode_index = *pod_index as usize;
    let str_index = misc_fun::epi_prefix_num(episode_index);
    let prefix_name = str_index.to_owned() + "_" + &file_dot_type;
    prefix_name
}

fn file_with_type(actual_title: &String, actual_url: &String) -> String {
    let file_type = misc_fun::file_type_real(actual_url.to_string());
    let file_dot_type = actual_title.to_owned() + "." + file_type.as_str();
    file_dot_type
}
