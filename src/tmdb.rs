use crate::models::{HomelabStatus, MediaItem, MediaType};
use anyhow::{Context, Result};
use chrono::Utc;
use reqwest::Client;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use tracing::{debug, info, warn};

pub struct TmdbClient {
    client: Client,
    api_key: String,
    base_url: String,
    max_pages: u32,
}

#[derive(Debug, Deserialize)]
struct DiscoverMovieResponse {
    #[serde(default)]
    results: Vec<TmdbMovieBasic>,
    #[serde(default)]
    total_pages: u32,
}

#[derive(Debug, Clone, Deserialize)]
struct TmdbMovieBasic {
    id: i64,
    #[serde(default)]
    title: String,
    #[serde(default)]
    original_title: String,
    #[serde(default)]
    original_language: String,
    #[serde(default)]
    overview: String,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
    #[serde(default)]
    release_date: String,
    #[serde(default)]
    vote_average: f64,
    #[serde(default)]
    vote_count: i64,
    #[serde(default)]
    popularity: f64,
    #[serde(default)]
    genre_ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
struct DiscoverTvResponse {
    #[serde(default)]
    results: Vec<TmdbTvBasic>,
    #[serde(default)]
    total_pages: u32,
}

#[derive(Debug, Clone, Deserialize)]
struct TmdbTvBasic {
    id: i64,
    #[serde(default)]
    name: String,
    #[serde(default)]
    original_name: String,
    #[serde(default)]
    original_language: String,
    #[serde(default)]
    overview: String,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
    #[serde(default)]
    first_air_date: String,
    #[serde(default)]
    vote_average: f64,
    #[serde(default)]
    vote_count: i64,
    #[serde(default)]
    popularity: f64,
    #[serde(default)]
    genre_ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
struct MovieDetailResponse {
    #[serde(default)]
    title: String,
    #[serde(default)]
    overview: String,
    #[serde(default)]
    genres: Vec<GenreObj>,
    videos: Option<VideosContainer>,
    #[serde(rename = "watch/providers")]
    watch_providers: Option<WatchProvidersContainer>,
    translations: Option<TranslationsContainer>,
    release_dates: Option<ReleaseDatesContainer>,
}

#[derive(Debug, Deserialize)]
struct TvDetailResponse {
    #[serde(default)]
    name: String,
    #[serde(default)]
    overview: String,
    #[serde(default)]
    genres: Vec<GenreObj>,
    #[serde(default)]
    number_of_seasons: Option<i64>,
    #[serde(default)]
    seasons: Vec<TvSeasonObj>,
    #[serde(default)]
    last_air_date: Option<String>,
    next_episode_to_air: Option<TvEpisodeObj>,
    #[serde(default)]
    networks: Vec<NetworkObj>,
    videos: Option<VideosContainer>,
    #[serde(rename = "watch/providers")]
    watch_providers: Option<WatchProvidersContainer>,
    translations: Option<TranslationsContainer>,
}

#[derive(Debug, Deserialize)]
struct GenreObj {
    name: String,
}

#[derive(Debug, Deserialize)]
struct NetworkObj {
    name: String,
}

#[derive(Debug, Deserialize)]
struct TvSeasonObj {
    season_number: i64,
    air_date: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TvEpisodeObj {
    air_date: Option<String>,
    season_number: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct VideosContainer {
    #[serde(default)]
    results: Vec<VideoObj>,
}

#[derive(Debug, Deserialize)]
struct VideoObj {
    key: String,
    site: String,
    #[serde(rename = "type")]
    video_type: String,
}

#[derive(Debug, Deserialize)]
struct WatchProvidersContainer {
    #[serde(default)]
    results: HashMap<String, CountryWatchProvider>,
}

#[derive(Debug, Deserialize)]
struct CountryWatchProvider {
    #[serde(default)]
    flatrate: Vec<ProviderObj>,
    #[serde(default)]
    rent: Vec<ProviderObj>,
    #[serde(default)]
    buy: Vec<ProviderObj>,
}

#[derive(Debug, Deserialize)]
struct ProviderObj {
    provider_name: String,
}

#[derive(Debug, Deserialize)]
struct TranslationsContainer {
    #[serde(default)]
    translations: Vec<TranslationObj>,
}

#[derive(Debug, Deserialize)]
struct TranslationObj {
    iso_639_1: String,
    data: Option<TranslationData>,
}

#[derive(Debug, Deserialize)]
struct TranslationData {
    overview: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ReleaseDatesContainer {
    #[serde(default)]
    results: Vec<CountryReleaseDates>,
}

#[derive(Debug, Deserialize)]
struct CountryReleaseDates {
    iso_3166_1: String,
    #[serde(default)]
    release_dates: Vec<ReleaseDateEntry>,
}

#[derive(Debug, Deserialize)]
struct ReleaseDateEntry {
    #[serde(rename = "type")]
    release_type: i32,
}

impl TmdbClient {
    pub fn new(api_key: String, base_url: String, max_pages: u32) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(20))
            .user_agent("BollySeerr/0.1.0")
            .build()
            .expect("failed to build reqwest client");

        Self {
            client,
            api_key,
            base_url: base_url.trim_end_matches('/').to_string(),
            max_pages,
        }
    }

    pub async fn discover_all(&self, min_date: &str, max_date: &str) -> Result<Vec<MediaItem>> {
        let mut items = Vec::new();

        // 1. Discover original Hindi movies
        match self
            .discover_movies_by_lang("hi", false, min_date, max_date, 0)
            .await
        {
            Ok(movies) => {
                info!("Discovered {} original Hindi movies", movies.len());
                items.extend(movies);
            }
            Err(e) => warn!("Failed to discover Hindi movies: {e:#}"),
        }

        // 2. Discover Pan-India movies (Telugu, Tamil, Kannada, Malayalam) that also release in Hindi
        for lang in ["te", "ta", "kn", "ml"] {
            match self
                .discover_movies_by_lang(lang, true, min_date, max_date, 5)
                .await
            {
                Ok(pan_movies) => {
                    info!(
                        "Discovered {} Pan-India ({lang}) movies with Hindi release",
                        pan_movies.len()
                    );
                    items.extend(pan_movies);
                }
                Err(e) => warn!("Failed to discover Pan-India ({lang}) movies: {e:#}"),
            }
        }

        // 3. Discover Hindi & Pan-India OTT Web Series (excluding daily soaps/talk/reality/news)
        match self
            .discover_web_series("hi", false, min_date, max_date)
            .await
        {
            Ok(series) => {
                info!("Discovered {} original Hindi web series", series.len());
                items.extend(series);
            }
            Err(e) => warn!("Failed to discover Hindi web series: {e:#}"),
        }

        for lang in ["te", "ta", "ml", "kn"] {
            match self
                .discover_web_series(lang, true, min_date, max_date)
                .await
            {
                Ok(pan_series) => {
                    info!(
                        "Discovered {} Pan-India ({lang}) web series with Hindi release",
                        pan_series.len()
                    );
                    items.extend(pan_series);
                }
                Err(e) => warn!("Failed to discover Pan-India ({lang}) web series: {e:#}"),
            }
        }

        // Deduplicate by (tmdb_id, media_type)
        let mut seen = HashSet::new();
        let mut deduped = Vec::new();
        for item in items {
            if seen.insert((item.tmdb_id, item.media_type.as_str())) {
                deduped.push(item);
            }
        }

        Ok(deduped)
    }

    async fn discover_movies_by_lang(
        &self,
        lang: &str,
        require_hindi_translation: bool,
        min_date: &str,
        max_date: &str,
        min_votes: i64,
    ) -> Result<Vec<MediaItem>> {
        let mut basics = Vec::new();
        let max_pages = if require_hindi_translation {
            self.max_pages.min(4)
        } else {
            self.max_pages
        };

        for page in 1..=max_pages {
            let url = format!("{}/discover/movie", self.base_url);
            let page_str = page.to_string();
            let min_votes_str = min_votes.to_string();
            let mut query = vec![
                ("api_key", self.api_key.as_str()),
                ("language", "en-IN"),
                ("region", "IN"),
                ("with_original_language", lang),
                ("primary_release_date.gte", min_date),
                ("primary_release_date.lte", max_date),
                ("sort_by", "primary_release_date.desc"),
                ("page", page_str.as_str()),
            ];
            if min_votes > 0 {
                query.push(("vote_count.gte", min_votes_str.as_str()));
            }

            let resp = self
                .client
                .get(&url)
                .query(&query)
                .send()
                .await
                .context("TMDB discover/movie request failed")?;

            if !resp.status().is_success() {
                warn!("TMDB discover/movie returned HTTP {}", resp.status());
                break;
            }

            let data: DiscoverMovieResponse = resp.json().await?;
            let total_pages = data.total_pages;
            basics.extend(data.results);

            if page >= total_pages || total_pages == 0 {
                break;
            }
        }

        let mut enriched = Vec::new();
        for b in basics {
            if b.release_date.is_empty()
                || b.release_date.as_str() < min_date
                || b.release_date.as_str() > max_date
            {
                continue;
            }

            match self.enrich_movie(b, require_hindi_translation).await {
                Ok(Some(item)) => enriched.push(item),
                Ok(None) => debug!("Skipped Pan-India movie without Hindi translation"),
                Err(e) => warn!("Failed to enrich movie: {e:#}"),
            }
        }

        Ok(enriched)
    }

    async fn enrich_movie(
        &self,
        basic: TmdbMovieBasic,
        require_hindi_translation: bool,
    ) -> Result<Option<MediaItem>> {
        let url = format!("{}/movie/{}", self.base_url, basic.id);
        let resp = self
            .client
            .get(&url)
            .query(&[
                ("api_key", self.api_key.as_str()),
                ("language", "en-IN"),
                (
                    "append_to_response",
                    "videos,watch/providers,translations,release_dates",
                ),
            ])
            .send()
            .await;

        let mut genres = map_genre_ids(&basic.genre_ids);
        let mut ott_providers = Vec::new();
        let mut release_type = "Theatrical".to_string();
        let mut trailer_key = None;
        let mut overview = basic.overview.clone();
        let mut title = if basic.title.is_empty() {
            basic.original_title.clone()
        } else {
            basic.title.clone()
        };
        let mut has_hindi_translation = basic.original_language == "hi";

        if let Ok(r) = resp
            && r.status().is_success()
            && let Ok(detail) = r.json::<MovieDetailResponse>().await
        {
            if !detail.title.is_empty() {
                title = detail.title;
            }
            if !detail.overview.is_empty() {
                overview = detail.overview;
            }
            if !detail.genres.is_empty() {
                genres = detail.genres.into_iter().map(|g| g.name).collect();
            }

            if let Some(translations) = detail.translations {
                for t in translations.translations {
                    if t.iso_639_1 == "hi" {
                        has_hindi_translation = true;
                        if overview.is_empty()
                            && let Some(d) = t.data
                            && let Some(ov) = d.overview.filter(|s| !s.is_empty())
                        {
                            overview = ov;
                        }
                    }
                }
            }

            if let Some(wp) = detail.watch_providers {
                ott_providers = extract_ott_providers(&wp.results);
            }

            if let Some(rd) = detail.release_dates {
                for c in rd.results {
                    if c.iso_3166_1 == "IN" {
                        let has_digital = c.release_dates.iter().any(|d| d.release_type == 4);
                        let has_theatrical = c
                            .release_dates
                            .iter()
                            .any(|d| d.release_type == 2 || d.release_type == 3);
                        if has_digital && !has_theatrical {
                            release_type = "OTT Direct".to_string();
                        } else if has_digital && has_theatrical {
                            release_type = "Theatrical + OTT".to_string();
                        }
                    }
                }
            }

            if !ott_providers.is_empty() && release_type == "Theatrical" {
                release_type = "OTT / Streaming".to_string();
            }

            if let Some(vids) = detail.videos {
                trailer_key = extract_youtube_trailer(&vids.results);
            }
        }

        if require_hindi_translation && !has_hindi_translation {
            return Ok(None);
        }

        Ok(Some(MediaItem {
            tmdb_id: basic.id,
            media_type: MediaType::Movie,
            title,
            original_title: basic.original_title,
            original_language: basic.original_language.clone(),
            is_hindi_dub: basic.original_language != "hi",
            overview,
            poster_path: basic.poster_path,
            backdrop_path: basic.backdrop_path,
            release_date: basic.release_date,
            vote_average: basic.vote_average,
            vote_count: basic.vote_count,
            popularity: basic.popularity,
            genres,
            ott_providers,
            release_type,
            trailer_key,
            season_count: None,
            latest_season_number: None,
            homelab_status: HomelabStatus::Unrequested,
            radarr_id: None,
            sonarr_id: None,
            seerr_request_id: None,
            updated_at: Utc::now().to_rfc3339(),
        }))
    }

    async fn discover_web_series(
        &self,
        lang: &str,
        require_hindi_translation: bool,
        min_date: &str,
        max_date: &str,
    ) -> Result<Vec<MediaItem>> {
        // Excluded daily TV genres:
        // 10766 = Soap, 10764 = Reality, 10767 = Talk, 10763 = News
        let excluded_genres = "10766,10764,10767,10763";
        let max_pages = if require_hindi_translation {
            self.max_pages.min(3)
        } else {
            self.max_pages.min(8)
        };

        let mut basics = Vec::new();

        // Query A: New web series premieres (first_air_date within window)
        for page in 1..=max_pages {
            let url = format!("{}/discover/tv", self.base_url);
            let page_str = page.to_string();
            let query = [
                ("api_key", self.api_key.as_str()),
                ("language", "en-IN"),
                ("with_original_language", lang),
                ("without_genres", excluded_genres),
                ("first_air_date.gte", min_date),
                ("first_air_date.lte", max_date),
                ("sort_by", "first_air_date.desc"),
                ("page", page_str.as_str()),
            ];

            let resp = self.client.get(&url).query(&query).send().await?;
            if !resp.status().is_success() {
                break;
            }
            let data: DiscoverTvResponse = resp.json().await?;
            let total_pages = data.total_pages;
            basics.extend(data.results);
            if page >= total_pages || total_pages == 0 {
                break;
            }
        }

        // Query B: Returning web series airing new episodes/seasons in window
        for page in 1..=max_pages.min(4) {
            let url = format!("{}/discover/tv", self.base_url);
            let page_str = page.to_string();
            let query = [
                ("api_key", self.api_key.as_str()),
                ("language", "en-IN"),
                ("with_original_language", lang),
                ("without_genres", excluded_genres),
                ("air_date.gte", min_date),
                ("air_date.lte", max_date),
                ("sort_by", "popularity.desc"),
                ("page", page_str.as_str()),
            ];

            let resp = self.client.get(&url).query(&query).send().await?;
            if !resp.status().is_success() {
                break;
            }
            let data: DiscoverTvResponse = resp.json().await?;
            let total_pages = data.total_pages;
            basics.extend(data.results);
            if page >= total_pages || total_pages == 0 {
                break;
            }
        }

        // Deduplicate basics by id
        let mut seen = HashSet::new();
        basics.retain(|b| seen.insert(b.id));

        let mut enriched = Vec::new();
        for b in basics {
            // Double-check soap/reality exclusion by genre_ids
            if b.genre_ids
                .iter()
                .any(|id| matches!(id, 10766 | 10764 | 10767 | 10763))
            {
                continue;
            }

            match self
                .enrich_tv(b, require_hindi_translation, min_date, max_date)
                .await
            {
                Ok(Some(item)) => enriched.push(item),
                Ok(None) => {}
                Err(e) => warn!("Failed to enrich TV series: {e:#}"),
            }
        }

        Ok(enriched)
    }

    async fn enrich_tv(
        &self,
        basic: TmdbTvBasic,
        require_hindi_translation: bool,
        min_date: &str,
        max_date: &str,
    ) -> Result<Option<MediaItem>> {
        let url = format!("{}/tv/{}", self.base_url, basic.id);
        let resp = self
            .client
            .get(&url)
            .query(&[
                ("api_key", self.api_key.as_str()),
                ("language", "en-IN"),
                ("append_to_response", "videos,watch/providers,translations"),
            ])
            .send()
            .await;

        let mut genres = map_genre_ids(&basic.genre_ids);
        let mut ott_providers = Vec::new();
        let mut trailer_key = None;
        let mut overview = basic.overview.clone();
        let mut title = if basic.name.is_empty() {
            basic.original_name.clone()
        } else {
            basic.name.clone()
        };
        let mut season_count = None;
        let mut latest_season_number = None;
        let mut effective_date = basic.first_air_date.clone();
        let mut has_hindi_translation = basic.original_language == "hi";

        if let Ok(r) = resp
            && r.status().is_success()
            && let Ok(detail) = r.json::<TvDetailResponse>().await
        {
            // Reject if genres include Soap/Reality/Talk/News
            if detail.genres.iter().any(|g| {
                let gl = g.name.to_lowercase();
                gl.contains("soap")
                    || gl.contains("reality")
                    || gl.contains("talk")
                    || gl.contains("news")
            }) {
                return Ok(None);
            }

            if !detail.name.is_empty() {
                title = detail.name;
            }
            if !detail.overview.is_empty() {
                overview = detail.overview;
            }
            if !detail.genres.is_empty() {
                genres = detail.genres.into_iter().map(|g| g.name).collect();
            }

            season_count = detail.number_of_seasons;

            // Determine effective release date from seasons or next_episode_to_air
            for s in &detail.seasons {
                if s.season_number > 0 {
                    latest_season_number = Some(s.season_number);
                    if let Some(ref ad) = s.air_date
                        && !ad.is_empty()
                        && ad.as_str() >= min_date
                        && ad.as_str() <= max_date
                    {
                        effective_date = ad.clone();
                    }
                }
            }

            if let Some(next_ep) = detail.next_episode_to_air {
                if let Some(ad) = next_ep.air_date
                    && !ad.is_empty()
                    && ad.as_str() >= min_date
                    && ad.as_str() <= max_date
                {
                    effective_date = ad;
                }
                if let Some(sn) = next_ep.season_number {
                    latest_season_number = Some(sn);
                }
            } else if effective_date.as_str() < min_date
                && let Some(last_ad) = detail.last_air_date
                && last_ad.as_str() >= min_date
                && last_ad.as_str() <= max_date
            {
                effective_date = last_ad;
            }

            if let Some(translations) = detail.translations {
                for t in translations.translations {
                    if t.iso_639_1 == "hi" {
                        has_hindi_translation = true;
                    }
                }
            }

            for net in detail.networks {
                if !ott_providers.contains(&net.name) {
                    ott_providers.push(net.name);
                }
            }

            if let Some(wp) = detail.watch_providers {
                for p in extract_ott_providers(&wp.results) {
                    if !ott_providers.contains(&p) {
                        ott_providers.push(p);
                    }
                }
            }

            if let Some(vids) = detail.videos {
                trailer_key = extract_youtube_trailer(&vids.results);
            }
        }

        if require_hindi_translation && !has_hindi_translation {
            return Ok(None);
        }

        if effective_date.is_empty()
            || effective_date.as_str() < min_date
            || effective_date.as_str() > max_date
        {
            return Ok(None);
        }

        let release_type = if let Some(sn) = latest_season_number {
            if sn > 1 {
                format!("Web Series • S{sn}")
            } else {
                "Web Series • S1".to_string()
            }
        } else {
            "Web Series".to_string()
        };

        Ok(Some(MediaItem {
            tmdb_id: basic.id,
            media_type: MediaType::Tv,
            title,
            original_title: basic.original_name,
            original_language: basic.original_language.clone(),
            is_hindi_dub: basic.original_language != "hi",
            overview,
            poster_path: basic.poster_path,
            backdrop_path: basic.backdrop_path,
            release_date: effective_date,
            vote_average: basic.vote_average,
            vote_count: basic.vote_count,
            popularity: basic.popularity,
            genres,
            ott_providers,
            release_type,
            trailer_key,
            season_count,
            latest_season_number,
            homelab_status: HomelabStatus::Unrequested,
            radarr_id: None,
            sonarr_id: None,
            seerr_request_id: None,
            updated_at: Utc::now().to_rfc3339(),
        }))
    }
}

fn extract_ott_providers(map: &HashMap<String, CountryWatchProvider>) -> Vec<String> {
    let mut out = Vec::new();
    for region in ["IN", "GB", "US"] {
        if let Some(country) = map.get(region) {
            for p in country
                .flatrate
                .iter()
                .chain(country.rent.iter())
                .chain(country.buy.iter())
            {
                let normalized = normalize_provider_name(&p.provider_name);
                if !out.contains(&normalized) {
                    out.push(normalized);
                }
            }
        }
    }
    out.truncate(4);
    out
}

fn normalize_provider_name(name: &str) -> String {
    let lower = name.to_lowercase();
    if lower.contains("netflix") {
        "Netflix".to_string()
    } else if lower.contains("amazon") || lower.contains("prime video") {
        "Prime Video".to_string()
    } else if lower.contains("hotstar") || lower.contains("jio") {
        "JioHotstar".to_string()
    } else if lower.contains("sonyliv") || lower.contains("sony liv") {
        "SonyLIV".to_string()
    } else if lower.contains("zee5") || lower.contains("zee 5") {
        "ZEE5".to_string()
    } else if lower.contains("apple tv") {
        "Apple TV+".to_string()
    } else if lower.contains("mx player") {
        "MX Player".to_string()
    } else if lower.contains("lionsgate") {
        "Lionsgate Play".to_string()
    } else {
        name.to_string()
    }
}

fn extract_youtube_trailer(videos: &[VideoObj]) -> Option<String> {
    videos
        .iter()
        .find(|v| {
            v.site.eq_ignore_ascii_case("YouTube") && v.video_type.eq_ignore_ascii_case("Trailer")
        })
        .or_else(|| {
            videos.iter().find(|v| {
                v.site.eq_ignore_ascii_case("YouTube")
                    && v.video_type.eq_ignore_ascii_case("Teaser")
            })
        })
        .map(|v| v.key.clone())
}

fn map_genre_ids(ids: &[i64]) -> Vec<String> {
    ids.iter()
        .filter_map(|id| match id {
            28 => Some("Action"),
            12 => Some("Adventure"),
            16 => Some("Animation"),
            35 => Some("Comedy"),
            80 => Some("Crime"),
            99 => Some("Documentary"),
            18 => Some("Drama"),
            10751 => Some("Family"),
            14 => Some("Fantasy"),
            36 => Some("History"),
            27 => Some("Horror"),
            10402 => Some("Music"),
            9648 => Some("Mystery"),
            10749 => Some("Romance"),
            878 => Some("Sci-Fi"),
            53 => Some("Thriller"),
            10752 => Some("War"),
            10759 => Some("Action & Adventure"),
            10765 => Some("Sci-Fi & Fantasy"),
            10768 => Some("War & Politics"),
            _ => None,
        })
        .map(std::string::ToString::to_string)
        .collect()
}
