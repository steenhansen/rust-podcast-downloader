#[allow(unused)]
use log::{debug, info, trace, warn};
use ratatui::layout::Rect;

pub const NEW_URL_AREA: ratatui::layout::Rect = Rect {
    x: 0,
    y: 6,
    width: 80,
    height: 3,
};

pub const NEW_NAME_AREA: ratatui::layout::Rect = Rect {
    x: 0,
    y: 2,
    width: 35,
    height: 3,
};

pub const ADD_PODCAST_AREA: ratatui::layout::Rect = Rect {
    x: 36,
    y: 1,
    width: 13,
    height: 3,
};

pub const TITLE_AREA: ratatui::layout::Rect = Rect {
    x: 0,
    y: 0,
    width: 30,
    height: 1,
};

pub const START_X_EPISODE: u16 = 35;
pub const START_Y_EPISODE: u16 = 9;

pub const START_X_PODCAST: u16 = 0;
pub const START_Y_PODCAST: u16 = 9;
pub const WIDTH_PODCAST: u16 = 33;

pub const RADIO_Y_START: u16 = 1;
pub const RADIO_AREA: ratatui::layout::Rect = Rect {
    x: 51,
    y: RADIO_Y_START,
    width: 27,
    height: 5,
};
