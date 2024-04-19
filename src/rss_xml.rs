#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::const_globals;
use crate::misc_fun;
use crate::the_types;
use regex::Regex;
use roxmltree::Children;

fn search_textss(the_text: String) -> String {
    //  <  href="http://www.artinnaturephotography.com/images/xl/salt-point-california-coast-20210808_0229.jpg"  >
    //  <  src='https://photojournal.jpl.nasa.gov/thumb/PIA26312.jpg'  >
    //    let first_jpeg = Regex::new(r####"=('|")[^"]*.jpg("|')"####).unwrap();
    let first_jpeg = Regex::new(const_globals::FIND_PICTURES).unwrap();

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

fn item_title_url(an_item: Children) -> Option<(String, String)> {
    let mut title_name = "";
    let mut the_url: &str = "";
    let mut the_text: String = "".to_string();
    for item_child in an_item {
        let sub_name = item_child.tag_name();
        if sub_name == "title".into() {
            title_name = item_child.text().unwrap();
        }
        if sub_name == "enclosure".into() {
            the_url = item_child.attribute("url").unwrap();
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
        let url_string = search_textss(the_text);
        return Some((title_string, url_string));
    }

    if title_name != "" && the_url != "" {
        let url_string = the_url.to_string();
        return Some((title_string, url_string));
    }
    None
}

pub fn negative_titles_urls(an_string: String) -> the_types::Result<Vec<(i32, String, String)>> {
    let mut titles_and_urls: Vec<(i32, String, String)> = Vec::new();
    let real_bytes = an_string.as_str();
    let _doc = match roxmltree::Document::parse(real_bytes) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };
    let doc = roxmltree::Document::parse(real_bytes).unwrap();
    let mut pod_index = 0;
    for node in doc.descendants() {
        if node.is_element() {
            let tag_name = node.tag_name();
            if tag_name == "item".into() {
                let the_children = node.children();
                let all_kids = the_children.into_iter();
                let title_and_url = item_title_url(all_kids);
                let (actual_title, actual_url) = (title_and_url).unwrap();
                let bad_thruple = (pod_index, actual_title, actual_url);
                titles_and_urls.push(bad_thruple);
                pod_index += 1;
            }
        }
    }
    Ok(titles_and_urls)
}

pub fn positive_titles_urls(
    titles_and_urls: Vec<(i32, String, String)>,
) -> Vec<(i32, String, String)> {
    let num_pods = titles_and_urls.len() as i32;
    let mut indexed_titles_and_urls: Vec<(i32, String, String)> = Vec::new();
    for title_and_url in titles_and_urls {
        let (pod_index, actual_title, actual_url) = title_and_url;
        let neg_pod_index = pod_index - num_pods;
        let good_pod_index = neg_pod_index.abs();
        let dashed_title = misc_fun::clean_title(actual_title);
        let good_thruple = (good_pod_index, dashed_title, actual_url);
        indexed_titles_and_urls.push(good_thruple);
    }
    indexed_titles_and_urls
}
