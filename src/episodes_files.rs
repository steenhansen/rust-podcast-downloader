#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::file_status;
use crate::type_partial_range_iter;

use reqwest::blocking::Client;

use reqwest::header::CONTENT_LENGTH;
use reqwest::header::RANGE;

use reqwest::StatusCode;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::str::FromStr;
use std::time::Duration;

pub fn read_episode_dir(selected_podcast: &str) -> HashMap<String, String> {
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
                    let k_real_file = String::from(real_file_str);
                    let v_real_file = String::from(real_file_str);
                    local_episodes.insert(k_real_file, v_real_file);
                }
                None => {}
            }
        }
    }

    local_episodes
}

pub fn media_length(media_url: &str) -> type_partial_range_iter::Result<u64> {
    let client = reqwest::blocking::Client::new();
    let response = client.head(media_url).send()?;
    let media_size = response
        .headers()
        .get(CONTENT_LENGTH)
        .ok_or("response doesn't include the content length")?;
    let size_str = media_size.to_str().unwrap();
    let len_media = u64::from_str(size_str)
        .map_err(|_| "invalid Content-Length header")
        .unwrap();
    Ok(len_media)
}
//   https://docs.rs/reqwest/0.11.3/reqwest/struct.Response.html#method.chunk

pub fn read_file(
    sel_podcast: String,
    file_name: String,
    url_episode: String,
) -> type_partial_range_iter::Result<()> {
    let len_media = media_length(&url_episode);
    let file_size = match len_media {
        Ok(len_media) => len_media,
        Err(_e) => 350_000_000, // one billion bytes if no known size
    };
    let client = reqwest::blocking::Client::new();
    let local_file = format!("{}/{}", sel_podcast, file_name);

    file_status::change_status(&local_file, 0);

    const CHUNK_SIZE: u32 = 1_000_000; // 134
    let _ = media_chunk(file_size, CHUNK_SIZE, client, url_episode, local_file);
    Ok(())
}

// https://rust-lang-nursery.github.io/rust-cookbook/web/clients/download.html#make-a-partial-download-with-http-range-headers
pub fn media_chunk(
    file_size: u64,
    chunk_size: u32,
    client: Client,
    url_episode: String,
    local_file: String,
) -> type_partial_range_iter::Result<()> {
    let mut output_file = File::create(&local_file)?;
    let timeout_duration = Duration::from_secs(100);
    let xx = type_partial_range_iter::PartialRangeIter::new(0, file_size - 1, chunk_size)?;
    let mut ind = 0;
    for range in xx {
        ind += 1;
        let byte_count = chunk_size * ind;
        let mut response_chunk = client
            .get(&url_episode)
            .header(RANGE, &range)
            .timeout(timeout_duration)
            .send()?;

        file_status::change_status(&local_file, byte_count);

        let status = response_chunk.status();
        if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
            error_chain::bail!("Unexpected server response: {}", status)
        }
        std::io::copy(&mut response_chunk, &mut output_file)?;
    }
    file_status::remove_status(&local_file);

    Ok(())
}
