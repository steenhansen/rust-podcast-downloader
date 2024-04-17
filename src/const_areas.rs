use ratatui::layout::Rect;

pub const NEW_PODCAST_URL: ratatui::layout::Rect = Rect {
    x: 0,
    y: 3,
    width: 80,
    height: 3,
};

pub const NEW_PODCAST_NAME: ratatui::layout::Rect = Rect {
    x: 0,
    y: 0,
    width: 35,
    height: 3,
};

pub const ADD_AREA: ratatui::layout::Rect = Rect {
    x: 36,
    y: 0,
    width: 15,
    height: 3,
};

pub const TITLE_AREA: ratatui::layout::Rect = Rect {
    x: 43,
    y: 0,
    width: 30,
    height: 1,
};

pub const START_X_EPISODE: u16 = 52; //0
pub const START_Y_EPISODE: u16 = 7;
pub const WIDTH_EPISODE: u16 = 75;
pub const HEIGHT_EPISODE: u16 = 8;

//pub const START_X_EPISODE_US: usize = START_X_EPISODE as usize;
pub const START_Y_EPISODE_US: usize = START_Y_EPISODE as usize;
//pub const WIDTH_EPISODE_US: usize = WIDTH_EPISODE as usize;
//pub const HEIGHT_EPISODE_US: usize = HEIGHT_EPISODE as usize;
pub const EPISODE_AREA: ratatui::layout::Rect = Rect {
    x: START_X_EPISODE,
    y: START_Y_EPISODE,
    width: WIDTH_EPISODE,
    height: HEIGHT_EPISODE,
};

pub const START_X_PODCAST: u16 = 0; //0
pub const START_Y_PODCAST: u16 = 7;
pub const WIDTH_PODCAST: u16 = 50;
pub const HEIGHT_PODCAST: u16 = 8;

//pub const START_X_PODCAST_US: usize = START_X_PODCAST as usize;
pub const START_Y_PODCAST_US: usize = START_Y_PODCAST as usize;
//pub const WIDTH_PODCAST_US: usize = WIDTH_PODCAST as usize;
//pub const HEIGHT_PODCAST_US: usize = HEIGHT_PODCAST as usize;

pub const PODCAST_AREA: ratatui::layout::Rect = Rect {
    x: START_X_PODCAST,
    y: START_Y_PODCAST,
    width: WIDTH_PODCAST,
    height: HEIGHT_PODCAST,
};
