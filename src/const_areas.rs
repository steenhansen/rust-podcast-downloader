use ratatui::layout::Rect;

pub const NEW_PODCAST_URL: ratatui::layout::Rect = Rect {
    x: 0,
    y: 6,
    width: 80,
    height: 3,
};

pub const NEW_PODCAST_NAME: ratatui::layout::Rect = Rect {
    x: 0,
    y: 2,
    width: 35,
    height: 3,
};

pub const ADD_AREA: ratatui::layout::Rect = Rect {
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

pub const START_X_EPISODE: u16 = 35; //0
pub const START_Y_EPISODE: u16 = 9;
// pub const WIDTH_EPISODE: u16 = 75;
// pub const HEIGHT_EPISODE: u16 = 8;

// pub const EPISODE_AREA: ratatui::layout::Rect = Rect {
//     x: START_X_EPISODE,
//     y: START_Y_EPISODE,
//     width: WIDTH_EPISODE,
//     height: HEIGHT_EPISODE,
// };

pub const START_X_PODCAST: u16 = 0; //0
pub const START_Y_PODCAST: u16 = 9;
pub const WIDTH_PODCAST: u16 = 33;
//pub const HEIGHT_PODCAST: u16 = 8;

// pub const PODCAST_AREA: ratatui::layout::Rect = Rect {
//     x: START_X_PODCAST,
//     y: START_Y_PODCAST,
//     width: WIDTH_PODCAST,
//     height: HEIGHT_PODCAST,
// };

pub const RADIO_Y_START: u16 = 1;
pub const RADIO_AREA: ratatui::layout::Rect = Rect {
    x: 51,
    y: RADIO_Y_START,
    width: 27,
    height: 5,
};
