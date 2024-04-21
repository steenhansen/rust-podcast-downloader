#[allow(unused)]
use log::{debug, info, trace, warn};
use ratatui::layout::Rect;

pub const NEW_URL_AREA: ratatui::layout::Rect = Rect {
    x: 0,
    y: 1,
    width: 74,
    height: 3,
};

pub const TITLE_AREA: ratatui::layout::Rect = Rect {
    x: 0,
    y: 0,
    width: 30,
    height: 1,
};

pub const NEW_NAME_AREA: ratatui::layout::Rect = Rect {
    x: 0,
    y: 4,
    width: 35,
    height: 3,
};

pub const ADD_PODCAST_AREA: ratatui::layout::Rect = Rect {
    x: 36,
    y: 4,
    width: 13,
    height: 3,
};

pub const ALL_PODCAST_AREA: ratatui::layout::Rect = Rect {
    x: 50,
    y: 4,
    width: 23,
    height: 3,
};

pub const START_X_EPISODE: u16 = 35;
pub const START_Y_EPISODE: u16 = 8;

pub const START_X_PODCAST: u16 = 0;
pub const START_Y_PODCAST: u16 = 8;
pub const WIDTH_PODCAST: u16 = 33;

pub const RADIO_Y_START: u16 = 1;
pub const RADIO_AREA: ratatui::layout::Rect = Rect {
    x: 76,
    y: RADIO_Y_START,
    width: 27,
    height: 5,
};

pub const PREFIX_AREA: ratatui::layout::Rect = Rect {
    x: 76,
    y: 7,
    width: 27,
    height: 1,
};
