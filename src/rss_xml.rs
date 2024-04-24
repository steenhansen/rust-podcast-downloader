#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::const_globals;
use crate::misc_fun;
use crate::the_types::*;

use regex::Regex;
use roxmltree::Children;
use std::error;

fn item_title_url(an_item: Children) -> Option<(EpisodeTitle, EpisodeUrl, EpisodeLength)> {
    let mut title_name = "";
    let mut the_url: &str = "";
    let mut the_text: String = "".to_string();
    let mut the_length: i32 = 0;
    for item_child in an_item {
        let sub_name = item_child.tag_name();
        if sub_name == "title".into() {
            title_name = item_child.text().expect("episode-missing-title-err");
        }
        if sub_name == "enclosure".into() {
            the_url = item_child
                .attribute("url")
                .expect("episode-missing-enclosure-err");

            let char_length = item_child
                .attribute("length")
                .expect("episode-missing-length-err");
            the_length = char_length
                .parse::<i32>()
                .expect("episode-length-not-int-err");
        }
        let option_text = item_child.text();
        match option_text {
            Some(some_text) => {
                the_text = the_text + some_text;
            }
            None => {}
        }
    }
    let title_string = title_name.to_string();
    if the_url == "" {
        let url_string = search_image_tag(the_text);
        return Some((title_string, url_string, the_length));
    }

    if title_name != "" && the_url != "" {
        let url_string = the_url.to_string();
        return Some((title_string, url_string, the_length));
    }
    None
}

pub fn dirty_titles_urls(
    an_string: String,
) -> Result<Vec<EpisodeMetadataTuple>, Box<dyn error::Error>> {
    let mut titles_and_urls: Vec<EpisodeMetadataTuple> = Vec::new();
    let real_bytes = an_string.as_str();
    let rss_doc = roxmltree::Document::parse(real_bytes)?; // NB, handle later and give error pop-up
    let mut episode_index = 0;
    for node in rss_doc.descendants() {
        if node.is_element() {
            let tag_name = node.tag_name();
            if tag_name == "item".into() {
                let the_children = node.children();
                let all_kids = the_children.into_iter();
                let title_and_url = item_title_url(all_kids);
                let (actual_title, actual_url, actual_len) =
                    (title_and_url).expect("item-title-url-err");
                let episode_metadata = (episode_index, actual_title, actual_url, actual_len);
                titles_and_urls.push(episode_metadata);
                episode_index += 1;
            }
        }
    }
    Ok(titles_and_urls)
}

pub fn clean_titles_urls(titles_and_urls: Vec<EpisodeMetadataTuple>) -> Vec<EpisodeMetadataTuple> {
    let num_pods = titles_and_urls.len() as i32;
    let mut indexed_titles_and_urls: Vec<EpisodeMetadataTuple> = Vec::new();
    for title_and_url in titles_and_urls {
        let (pod_index, actual_title, actual_url, actual_len) = title_and_url;
        let neg_pod_index = pod_index - num_pods;
        let good_pod_index = neg_pod_index.abs();
        let dashed_title = misc_fun::clean_title(actual_title);
        let episode_metadata = (good_pod_index, dashed_title, actual_url, actual_len);
        indexed_titles_and_urls.push(episode_metadata);
    }
    indexed_titles_and_urls
}

fn search_image_tag(the_text: String) -> String {
    //  <  href="http://www.artinnaturephotography.com/images/xl/salt-point-california-coast-20210808_0229.jpg"  >
    //  <  src='https://photojournal.jpl.nasa.gov/thumb/PIA26312.jpg'  >
    //    let first_jpeg = Regex::new(r####"=('|")[^"]*.jpg("|')"####)
    let first_jpeg = Regex::new(const_globals::FIND_PICTURES).expect("bad-image-regex-err");

    let possible_jpeg = first_jpeg.find(the_text.as_str());
    match possible_jpeg {
        Some(the_match) => {
            let match_jpg = the_match.as_str();
            let the_url = &match_jpg[2..match_jpg.len() - 1];
            return String::from(the_url);
        }
        None => return String::from(""),
    }
}
