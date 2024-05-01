//use http_req::response::Response;
#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::chunks::chunk_range;
use crate::consts::const_globals;
use crate::globals::g_current_active;
use crate::globals::g_pause_io;
use crate::globals::g_resource_speed;
use crate::globals::g_stop_io;

use reqwest::blocking::Client;
use reqwest::header::HeaderValue;
use reqwest::header::CONTENT_LENGTH;
use reqwest::header::RANGE;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::error;
use std::fs;
use std::fs::File;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

pub fn read_episode_dir(selected_podcast: &str) -> HashMap<String, String> {
    let episodes_dir = format!("{}{}", "./", selected_podcast);
    let mut local_episodes: HashMap<String, String> = HashMap::new();
    let dir_entries = fs::read_dir(episodes_dir).expect("read-episode-dir-err");
    for an_entry in dir_entries {
        let the_entry = an_entry.expect("episode-dir-entry-err");
        let the_path = the_entry.path();
        if the_path.is_file() {
            match &the_path.file_name() {
                Some(local_file) => {
                    let real_file_str = local_file.to_str().expect("episode-dir-fname-err");
                    if real_file_str != const_globals::RSS_TEXT_FILE {
                        let k_real_file = String::from(real_file_str);
                        let v_real_file = String::from(real_file_str);
                        local_episodes.insert(k_real_file, v_real_file);
                    }
                }
                None => {}
            }
        }
    }
    local_episodes
}

pub fn media_length(media_url: &str) -> Result<u64, Box<dyn error::Error>> {
    let client = reqwest::blocking::Client::new();
    let response = match client.head(media_url).send() {
        Ok(len_media) => len_media,
        Err(e) => return Err(Box::new(e)),
    };
    let media_size = response.headers().get(CONTENT_LENGTH);
    let header_size: &HeaderValue = match media_size {
        Some(size_head) => size_head,
        None => return Err("empty file").expect("reqwest-header-length-err"),
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

pub fn read_file(
    sel_podcast: String,
    file_name: String,
    url_episode: String,
) -> Result<bool, Box<dyn error::Error>> {
    let local_file = format!("{}/{}", sel_podcast, file_name);
    g_current_active::change_status(&local_file, const_globals::BYTE_COUNT_INIT);
    let len_media = media_length(&url_episode);
    let file_size = match len_media {
        Ok(len_media) => len_media,
        Err(_e) => 350_000_000, // one billion bytes if no known size
    };
    let client = reqwest::blocking::Client::new();
    let chunk_size = const_globals::CHUNK_SIZE;
    g_current_active::change_status(&local_file, const_globals::BYTE_COUNT_START);
    let finished_downloading =
        media_chunked(file_size, chunk_size, client, url_episode, local_file)
            .expect("chunked-file-err");
    Ok(finished_downloading)
}

pub fn media_chunked(
    file_size: u64,
    chunk_size: u32,
    client: Client,
    url_episode: String,
    local_file: String,
) -> Result<bool, Box<dyn error::Error>> {
    let working_file = local_file.clone() + const_globals::WORKING_FILE;
    let mut output_file = match File::create(&working_file) {
        Ok(output_file) => output_file,
        Err(e) => return Err(Box::new(e)),
    };
    let media_file_chunks = chunk_range::chunk_range_iter(0, file_size - 1, chunk_size);
    let mut chunk_index = 0;
    for file_section in media_file_chunks {
        if g_stop_io::is_stoping() {
            g_current_active::media_stoped(&local_file);
            fs::remove_file(working_file).expect("remove-killed-downloading");
            return Ok(false);
        }
        while speed_sleep() {}
        chunk_index += 1;
        let byte_count = chunk_size * chunk_index;
        let mut response_chunk = match client
            .get(&url_episode)
            .header(RANGE, &file_section)
            .timeout(const_globals::CHUNK_TIMEOUT)
            .send()
        {
            Ok(the_response) => the_response,
            Err(e) => {
                warn!("eeeeeeeeeeeeeeeeeeeeef {:?}", e);
                return Ok(false);
                //return Err(Box::new(e));
            }
        };

        g_current_active::change_status(&local_file, byte_count);
        let the_status = response_chunk.status();
        if !(the_status == StatusCode::OK || the_status == StatusCode::PARTIAL_CONTENT) {
            return Err(format!("Unexpected server response: {}", the_status))
                .expect("chunk-status-err");
        }
        std::io::copy(&mut response_chunk, &mut output_file).expect("read-write-err");
    }
    output_file.sync_all().expect("sync-file-err-2");
    fs::rename(working_file, &local_file).expect("work-rename-err");

    let finished_downloading = g_current_active::remove_status(&local_file);
    Ok(finished_downloading)
}

fn speed_sleep() -> bool {
    let cur_speed = g_resource_speed::get_speed();
    match cur_speed {
        0 => (),
        1 => thread::sleep(Duration::from_secs(const_globals::SLEEP_SEC_MED)),
        _ => thread::sleep(Duration::from_secs(const_globals::SLEEP_SEC_SLOW)),
    }
    let currently_paused = g_pause_io::is_cur_paused();
    if currently_paused {
        thread::sleep(Duration::from_secs(const_globals::PAUSE_SLEEP_SEC));
    }
    currently_paused
}
