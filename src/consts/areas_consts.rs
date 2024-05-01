#[allow(unused)]
use log::{debug, info, trace, warn};

use ratatui::layout::Rect;

pub const NEW_NAME_AREA: ratatui::layout::Rect = Rect {
    x: 0,
    y: 1,
    width: 30,
    height: 3,
};

pub const NEW_URL_AREA: ratatui::layout::Rect = Rect {
    x: 0,
    y: 4,
    width: 53,
    height: 3,
};

pub const NEW_PODCAST_AREA: ratatui::layout::Rect = Rect {
    x: 31,
    y: 1,
    width: 17,
    height: 3,
};

pub const EVERY_EPISODE_AREA: ratatui::layout::Rect = Rect {
    x: 49,
    y: 1,
    width: 24,
    height: 3,
};

pub const STOP_PODCAST_AREA: ratatui::layout::Rect = Rect {
    x: 54,
    y: 5,
    width: 18,
    height: 3,
};

pub const START_X_EPISODE: u16 = 35;
pub const START_Y_EPISODE: u16 = 8;

pub const START_X_PODCAST: u16 = 0;
pub const START_Y_PODCAST: u16 = 8;
pub const WIDTH_PODCAST: u16 = 33;

pub const RESOURCE_Y_START: u16 = 1;

pub const RESOURCE_AREA: ratatui::layout::Rect = Rect {
    x: 74,
    y: RESOURCE_Y_START,
    width: 24,
    height: 5,
};

pub const PAUSE_AREA: ratatui::layout::Rect = Rect {
    x: 74,
    y: 7,
    width: 27,
    height: 1,
};

pub const MIN_FEED_X_START: i16 = 30; //  https://www.nasa.gov/feeds/iotd-feed
