#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::components::podcasts::podcast_directory;
use crate::consts::consts_types::*;
use crate::files::file_xml;
use crate::misc::misc_fun;
use crate::state::state_app;

use std::collections::HashMap;
use std::error;

pub fn contents_episode_list(
    the_app: &mut state_app::DownApp,
) -> Result<(), Box<dyn error::Error>> {
    podcast_directory::directory_erase(the_app);
    let rss_feed = podcast_directory::directory_get_filename(the_app.selected_podcast.clone());
    let an_string = match podcast_directory::directory_read_rss(&rss_feed) {
        Ok(an_string) => an_string,
        Err(_e) => {
            let bad_url_err = format!("\n Error - {:?} is not valid URL", &rss_feed);
            return Err(bad_url_err.into());
        }
    };
    let neg_titles_urls = match file_xml::xml_dirty_titles_urls(an_string) {
        Ok(v) => v,
        Err(_e) => {
            let bad_xml_err = format!("\n Error - {:?} is not valid XML", &rss_feed);
            return Err(bad_xml_err.into());
        }
    };
    contents_clear_podcast_data(the_app);
    contents_slurp_data(the_app, neg_titles_urls);
    the_app.scrolled_episodes_pos = 0;
    the_app.state_scroll_episodes = the_app
        .state_scroll_episodes
        .content_length(the_app.ordered_episodes.len());

    Ok(())
}

fn contents_slurp_data(
    the_app: &mut state_app::DownApp,
    neg_titles_urls: Vec<EpisodeMetadataTuple>,
) {
    let pos_titles_urls = file_xml::xml_clean_titles_urls(neg_titles_urls);
    for title_and_url in &pos_titles_urls {
        let (pod_index, actual_title, actual_url, actual_len) = title_and_url;
        let mut file_dot_type = contents_file_with_type(actual_title, actual_url);
        file_dot_type = contents_add_working(file_dot_type, pod_index);
        the_app.ordered_episodes.push(file_dot_type.clone());
        the_app
            .episode_2_url
            .insert(file_dot_type.clone(), actual_url.clone());

        the_app
            .episode_2_len
            .insert(file_dot_type.clone(), *actual_len);
    }
}

fn contents_clear_podcast_data(the_app: &mut state_app::DownApp) {
    the_app.ordered_episodes = Vec::new();
    the_app.episode_2_url = HashMap::new();
    the_app.episode_2_len = HashMap::new();
}

fn contents_add_working(file_dot_type: String, pod_index: &i32) -> String {
    let episode_index = *pod_index as usize;
    let str_index = misc_fun::epi_prefix_num(episode_index);
    let prefix_name = str_index.to_owned() + "_" + &file_dot_type;
    prefix_name
}

fn contents_file_with_type(actual_title: &String, actual_url: &String) -> String {
    let file_type = misc_fun::file_type_real(actual_url.to_string());
    let file_dot_type = actual_title.to_owned() + "." + file_type.as_str();
    file_dot_type
}
