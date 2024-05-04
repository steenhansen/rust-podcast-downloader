#[allow(unused)]
use log::{debug, info, trace, warn};

use ratatui::layout::Rect;

pub const NEW_NAME_AREA: ratatui::layout::Rect = Rect {
    x: 0,
    y: 1,
    width: 30,
    height: 3,
};
pub const MAX_NAME_WIDTH: u16 = NEW_NAME_AREA.width - 3;

pub const NEW_URL_AREA: ratatui::layout::Rect = Rect {
    x: 0,
    y: 4,
    width: 54,
    height: 3,
};

pub const MAX_URL_WIDTH: u16 = 333;

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
    x: 55,
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
    width: 16,
    height: 5,
};

pub const PAUSE_AREA: ratatui::layout::Rect = Rect {
    x: 74,
    y: 7,
    width: 27,
    height: 1,
};

pub const MIN_FEED_X_START: i16 = 30; //  https://www.nasa.gov/feeds/iotd-feed

//
pub const HELP_SIZE_AREA: ratatui::layout::Rect = Rect {
    x: 0,
    y: 0,
    width: 50,
    height: 15,
};

pub const HELP_SIZE_OK: ratatui::layout::Rect = Rect {
    x: 0,
    y: 11,
    width: 6,
    height: 3,
};

pub const DIALOG_SURE_AREA: ratatui::layout::Rect = Rect {
    x: 0,
    y: 0,
    width: 63,
    height: 14,
};

pub const DIALOG_SURE_YES: ratatui::layout::Rect = Rect {
    x: 0,
    y: 5,
    width: 7,
    height: 3,
};
pub const DIALOG_SURE_NO: ratatui::layout::Rect = Rect {
    x: 0,
    y: 9,
    width: 7,
    height: 3,
};

pub const ERROR_SIZE_AREA: ratatui::layout::Rect = Rect {
    x: 0,
    y: 0,
    width: 49,
    height: 14,
};

pub const ERROR_SIZE_OK: ratatui::layout::Rect = Rect {
    x: 0,
    y: 10,
    width: 6,
    height: 3,
};
