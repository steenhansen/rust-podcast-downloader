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
    #[strum(to_string = "State001NoFocus")]
    StateNoFocus,
    #[strum(to_string = "State101NewPodcastUrl")]
    State001NewPodcastUrl,
    #[strum(to_string = "State102NewPodcastName")]
    State002NewPodcastName,
    #[strum(to_string = "State103ClickedAdd")]
    State003ClickedAdd,

    #[strum(to_string = "State201ReadingRss")]
    State101ReadingRss,
    #[strum(to_string = "State202ShowWaiting")]
    State102ShowWaiting,
    #[strum(to_string = "State203ShowEpisodes")]
    State103ShowEpisodes,
    #[strum(to_string = "State204UpdatedEpisodes")]
    State104UpdatedEpisodes,

    #[strum(to_string = "State301AllEpisodes")]
    State201AllEpisodes,
    #[strum(to_string = "State302SureAllEpisodes")]
    State202SureAllEpisodes,
    #[strum(to_string = "State303DownloadedingAll")]
    State203DownloadedingAll,
    #[strum(to_string = "State304AfterAll")]
    State204AfterAll,

    #[strum(to_string = "State401WaitForPopErrorClose")]
    State301WaitForPopErrorClose,

    #[strum(to_string = "State501DownloadPaused")]
    State401DownloadPaused,
    #[strum(to_string = "State502NotPaused")]
    State402NotPaused,
}
