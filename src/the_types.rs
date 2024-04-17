use strum::{Display, EnumIter, FromRepr};

#[derive(Debug, Default, Clone, Copy, Display, FromRepr, EnumIter, PartialEq)]
pub enum UiState {
    #[default]
    #[strum(to_string = "StateNoFocus")]
    StateNoFocus,
    #[strum(to_string = "State001NewPodcastUrl")]
    State001NewPodcastUrl,
    #[strum(to_string = "State002NewPodcastName")]
    State002NewPodcastName,
    #[strum(to_string = "State003ClickedAdd")]
    State003ClickedAdd,

    #[strum(to_string = "State101ReadingRss")]
    State101ReadingRss,
    #[strum(to_string = "State102ShowWaiting")]
    State102ShowWaiting,
    #[strum(to_string = "State103ShowEpisodes")]
    State103ShowEpisodes,
    #[strum(to_string = "State104UpdatedEpisodes")]
    State104UpdatedEpisodes,

    #[strum(to_string = "StateWaitForPopErrorClose")]
    StateWaitForPopErrorClose,

    #[strum(to_string = "StateLocalFiles")]
    StateLocalFiles,
}
use std::error;
pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
