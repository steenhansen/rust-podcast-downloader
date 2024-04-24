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

    #[strum(to_string = "State201AllEpisodes")]
    State201AllEpisodes,
    #[strum(to_string = "State202SureAllEpisodes")]
    State202SureAllEpisodes,
    #[strum(to_string = "State203DownloadedingAll")]
    State203DownloadedingAll,
    #[strum(to_string = "State204AfterAll")]
    State204AfterAll,

    #[strum(to_string = "State301WaitForPopErrorClose")]
    State301WaitForPopErrorClose,

    #[strum(to_string = "StateLocalFiles")]
    StateLocalFiles,
}
