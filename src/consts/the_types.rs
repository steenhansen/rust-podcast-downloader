use strum::{Display, EnumIter, FromRepr};

pub type PodcastName = String;
pub type PodcastUrl = String;

pub type EpisodeIndex = i32;
pub type EpisodeFilename = String;
pub type EpisodeTitle = String;
pub type EpisodeLength = i32;
pub type EpisodeUrl = String;

pub type EpisodeMetadataTuple = (EpisodeIndex, EpisodeTitle, EpisodeUrl, EpisodeLength);
pub type PodcastMetadataTuple = (PodcastName, EpisodeFilename, EpisodeUrl);

#[derive(Debug, Default, Clone, Copy, Display, FromRepr, EnumIter, PartialEq)]
pub enum UiState {
    #[default]
    #[strum(to_string = "StateNoFocus")]
    StateNoFocus,
    #[strum(to_string = "State001NewPodcastUrl")]
    State001NewPodcastUrl,
    #[strum(to_string = "State002NewPodcastName")]
    State002NewPodcastName,
    #[strum(to_string = "State003ClickedNew")]
    State003ClickedNew,

    #[strum(to_string = "State101ReadingRss")]
    State101ReadingRss,
    #[strum(to_string = "State102ShowWaiting")]
    State102ShowWaiting,
    #[strum(to_string = "State103ShowEpisodes")]
    State103ShowEpisodes,

    #[strum(to_string = "State201EveryEpisode")]
    State201EveryEpisode,
    #[strum(to_string = "State202SureEveryEpisode")]
    State202SureEveryEpisode,
    #[strum(to_string = "State203DownloadingEvery")]
    State203DownloadingEvery,

    #[strum(to_string = "State301WaitForPopErrorClose")]
    State301WaitForPopErrorClose,

    #[strum(to_string = "State401DownloadPaused")]
    State401DownloadPaused,

    #[strum(to_string = "State501Help")]
    State501Help,

    #[strum(to_string = "State601KillingDownloads")]
    State601KillingDownloads,
}
