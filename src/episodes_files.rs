//use http_req::response::Response;
#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::chunk_range;
use crate::const_globals;
use crate::g_current_active;
use crate::g_resource_speed;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_LENGTH;
use reqwest::header::RANGE;
use std::error;

use reqwest::StatusCode;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

pub fn read_episode_dir(selected_podcast: &str) -> HashMap<String, String> {
    // pub fn read_episode_dir(
    //     selected_podcast: &str,
    // ) -> Result<HashMap<String, String>, Box<dyn error::Error>> {
    let episodes_dir = format!("{}{}", "./", selected_podcast);
    let mut local_episodes: HashMap<String, String> = HashMap::new();
    let dir_entries = fs::read_dir(episodes_dir).unwrap();
    for an_entry in dir_entries {
        let the_entry = an_entry.expect("bard2");
        let the_path = the_entry.path();
        if the_path.is_file() {
            match &the_path.file_name() {
                Some(local_file) => {
                    let real_file_str = local_file.to_str().unwrap();
                    if real_file_str != const_globals::RSS_TEXT_FILE
                        && real_file_str != const_globals::INT_PREFIX_Y_N
                    {
                        let k_real_file = String::from(real_file_str);
                        let v_real_file = String::from(real_file_str);
                        // warn!("epi_filter v_real_file {:?}", v_real_file);
                        local_episodes.insert(k_real_file, v_real_file);
                    }
                }
                None => {}
            }
        }
    }
    //  Ok(local_episodes)
    local_episodes
}

use reqwest::header::HeaderValue;
pub fn media_length(media_url: &str) -> Result<u64, Box<dyn error::Error>> {
    let client = reqwest::blocking::Client::new();
    let response = match client.head(media_url).send() {
        Ok(len_media) => len_media,
        Err(e) => return Err(Box::new(e)),
    };
    let media_size = response.headers().get(CONTENT_LENGTH);
    let header_size: &HeaderValue = match media_size {
        Some(size_head) => size_head,
        None => return Err("empty file")?,
    };
    let header_str: &str = match header_size.to_str() {
        Ok(size_head) => size_head,
        Err(e) => return Err(Box::new(e)),
    };
    let len_media: u64 = match u64::from_str(header_str) {
        Ok(size_u64) => size_u64,
        Err(e) => return Err(Box::new(e)),
    };
    Ok(len_media)
}

//   https://docs.rs/reqwest/0.11.3/reqwest/struct.Response.html#method.chunk

pub fn read_file(
    sel_podcast: String,
    file_name: String,
    url_episode: String,
) -> Result<(), Box<dyn error::Error>> {
    let len_media = media_length(&url_episode);
    let file_size = match len_media {
        Ok(len_media) => len_media,
        Err(_e) => 350_000_000, // one billion bytes if no known size
    };
    let client = reqwest::blocking::Client::new();
    let local_file = format!("{}/{}", sel_podcast, file_name);
    g_current_active::change_status(&local_file, 0);
    let chunk_size = const_globals::CHUNK_SIZE;
    let _ = media_chunked(file_size, chunk_size, client, url_episode, local_file);
    Ok(())
}

// https://rust-lang-nursery.github.io/rust-cookbook/web/clients/download.html#make-a-partial-download-with-http-range-headers
pub fn media_chunked(
    file_size: u64,
    chunk_size: u32,
    client: Client,
    url_episode: String,
    local_file: String,
) -> Result<(), Box<dyn error::Error>> {
    let working_file = local_file.clone() + const_globals::WORKING_FILE;
    let mut output_file = match File::create(&working_file) {
        Ok(output_file) => output_file,
        Err(e) => return Err(Box::new(e)),
    };
    let media_file_sections = chunk_range::chunk_range_iter(0, file_size - 1, chunk_size);
    let mut ind = 0;
    for file_section in media_file_sections {
        speed_sleep();
        ind += 1;
        let byte_count = chunk_size * ind;
        let mut response_chunk = client
            .get(&url_episode)
            .header(RANGE, &file_section)
            .timeout(const_globals::CHUNK_TIMEOUT)
            .send()?;
        g_current_active::change_status(&local_file, byte_count);
        let the_status = response_chunk.status();
        if !(the_status == StatusCode::OK || the_status == StatusCode::PARTIAL_CONTENT) {
            return Err(format!("Unexpected server response: {}", the_status))?;
        }
        std::io::copy(&mut response_chunk, &mut output_file)?;
    }
    fs::rename(working_file, &local_file)?;
    g_current_active::remove_status(&local_file);
    Ok(())
}

fn speed_sleep() {
    let cur_speed = g_resource_speed::get_speed();
    match cur_speed {
        0 => return,
        1 => thread::sleep(Duration::from_secs(const_globals::SLEEP_SEC_MED)),
        _ => thread::sleep(Duration::from_secs(const_globals::SLEEP_SEC_SLOW)),
    }
}
