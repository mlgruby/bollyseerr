use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Movie,
    Tv,
}

impl MediaType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Movie => "movie",
            Self::Tv => "tv",
        }
    }

    pub fn parse_str(s: &str) -> Self {
        match s {
            "tv" | "series" | "show" => Self::Tv,
            _ => Self::Movie,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HomelabStatus {
    Unrequested,
    Requested,
    Monitored,
    Downloading,
    PartiallyAvailable,
    Available,
}

impl HomelabStatus {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Unrequested => "unrequested",
            Self::Requested => "requested",
            Self::Monitored => "monitored",
            Self::Downloading => "downloading",
            Self::PartiallyAvailable => "partially_available",
            Self::Available => "available",
        }
    }

    pub fn parse_str(s: &str) -> Self {
        match s {
            "requested" => Self::Requested,
            "monitored" => Self::Monitored,
            "downloading" => Self::Downloading,
            "partially_available" => Self::PartiallyAvailable,
            "available" => Self::Available,
            _ => Self::Unrequested,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub tmdb_id: i64,
    pub media_type: MediaType,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub is_hindi_dub: bool,
    pub overview: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub release_date: String,
    pub vote_average: f64,
    pub vote_count: i64,
    pub popularity: f64,
    pub genres: Vec<String>,
    pub ott_providers: Vec<String>,
    pub release_type: String,
    pub trailer_key: Option<String>,
    pub season_count: Option<i64>,
    pub latest_season_number: Option<i64>,
    pub homelab_status: HomelabStatus,
    pub radarr_id: Option<i64>,
    pub sonarr_id: Option<i64>,
    pub seerr_request_id: Option<i64>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub last_discovery_sync: Option<String>,
    pub last_status_sync: Option<String>,
    pub total_movies: usize,
    pub total_series: usize,
    pub released_count: usize,
    pub upcoming_count: usize,
    pub library_or_requested_count: usize,
    pub pruned_last_run: usize,
    pub is_syncing: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequestPayload {
    pub tmdb_id: i64,
    pub media_type: MediaType,
    pub seasons: Option<String>, // "all" or "latest" for TV
}

#[derive(Debug, Clone, Serialize)]
pub struct RequestResult {
    pub success: bool,
    pub status: HomelabStatus,
    pub message: String,
    pub via: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MediaCreditsAndKeywords {
    pub cast: Vec<CastMember>,
    pub crew: Vec<CrewMember>,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastMember {
    pub name: String,
    pub character: String,
    pub profile_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrewMember {
    pub name: String,
    pub job: String,
}
