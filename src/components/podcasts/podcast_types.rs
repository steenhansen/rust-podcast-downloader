#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::components::podcasts::podcast_happenings;
use crate::components::podcasts::podcast_paint;
use crate::consts::const_colors;
use crate::consts::const_globals;
use crate::consts::const_types;
use crate::state::state_app;

use ratatui::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub fn types_state_of_podcasts(
    console_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    app_dim: bool,
    is_downloading_paused: bool,
) {
    let elastic_pod_area = podcast_paint::paint_podcast_area(console_frame);
    let mut wait_color = const_colors::NORMAL_BORDER_COL;
    if is_downloading_paused {
        wait_color = const_colors::PAUSE_COLOR;
    } else if app_dim {
        wait_color = const_colors::DIMMED_BACKGROUND_WAIT;
    }
    podcast_paint::paint_pod_list(
        console_frame,
        elastic_pod_area,
        the_app,
        String::from("Choose Podcast to Download"),
        wait_color,
    );
    podcast_happenings::happening_podcasts_vscroll(console_frame, elastic_pod_area, the_app);
}

pub fn types_read_podcast_dir(root_dir: &str) -> HashMap<String, String> {
    let mut episodes_local: HashMap<String, String> = HashMap::new();
    let dir_entries = fs::read_dir(root_dir).expect("bad-dir-read-err");
    for an_entry in dir_entries {
        let the_entry = an_entry.expect("bad-dir-entry-err");
        let the_path = the_entry.path();
        if the_path.is_dir() {
            let my_str = the_path.as_path().display().to_string();
            let pod_name: String = my_str.chars().skip(2).collect();
            let rss_file_path =
                format!("{}{}/{}", root_dir, pod_name, const_globals::RSS_TEXT_FILE);
            if Path::new(&rss_file_path).is_file() {
                let mut file_handle = File::open(&rss_file_path).expect("file-open-err");
                let mut pod_url = String::new();
                file_handle
                    .read_to_string(&mut pod_url)
                    .expect("read-to-string-err");
                episodes_local.insert(pod_name, pod_url);
            }
        }
    }
    episodes_local
}

fn types_create_new_podcast(new_podcast_name: &str, contents_url: &str) -> Result<(), io::Error> {
    fs::create_dir(new_podcast_name).expect("create-dir-err");
    let f_name = format!("{}/{}", new_podcast_name, const_globals::RSS_TEXT_FILE);
    fs::write(f_name, contents_url).expect("create-new-write-err");
    Ok(())
}

pub fn types_create_pod_dir(the_app: &mut state_app::DownApp) -> Result<(), io::Error> {
    the_app.ui_state = const_types::UiState::StateNoFocus;
    types_create_new_podcast(&the_app.new_podcast_name, &the_app.new_podcast_url)
        .expect("make-dir-err");
    the_app.new_podcast_name = String::from("");
    the_app.new_podcast_url = String::from("");
    types_dirs_of_podcasts(the_app);
    Ok(())
}

pub fn types_dirs_of_podcasts(the_app: &mut state_app::DownApp) {
    let unordered_podcasts = types_read_podcast_dir(const_globals::ROOT_DIR);
    the_app.ordered_podcasts = <HashMap<String, String> as Clone>::clone(&unordered_podcasts)
        .into_iter()
        .map(|(p_name, _p_url)| p_name)
        .collect();
    the_app
        .ordered_podcasts
        .sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
}
