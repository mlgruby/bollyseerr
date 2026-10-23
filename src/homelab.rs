use crate::config::Config;
use crate::db::Database;
use crate::models::{DeleteResult, HomelabStatus, MediaType, RequestResult};
use anyhow::{Context, Result, anyhow};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, info, warn};

pub struct HomelabClient {
    client: Client,
    config: Config,
}

#[derive(Debug, Deserialize)]
struct RadarrMovie {
    id: i64,
    #[serde(rename = "tmdbId")]
    tmdb_id: i64,
    #[serde(default)]
    monitored: bool,
    #[serde(rename = "hasFile", default)]
    has_file: bool,
}

#[derive(Debug, Deserialize)]
struct RadarrRootFolder {
    path: String,
}

#[derive(Debug, Deserialize)]
struct RadarrQualityProfile {
    id: i64,
}

#[derive(Debug, Deserialize)]
struct SonarrSeries {
    id: i64,
    #[serde(rename = "tmdbId", default)]
    tmdb_id: Option<i64>,
    #[serde(default)]
    monitored: bool,
    statistics: Option<SonarrStats>,
}

#[derive(Debug, Deserialize)]
struct SonarrStats {
    #[serde(rename = "episodeFileCount", default)]
    episode_file_count: i64,
    #[serde(rename = "episodeCount", default)]
    episode_count: i64,
}

#[derive(Debug, Deserialize)]
struct SeerrRequestList {
    #[serde(default)]
    results: Vec<SeerrRequestItem>,
}

#[derive(Debug, Deserialize)]
struct SeerrRequestItem {
    id: i64,
    #[serde(rename = "type")]
    media_type: String,
    media: Option<SeerrMediaInfo>,
}

#[derive(Debug, Deserialize)]
struct SeerrMediaInfo {
    #[serde(rename = "tmdbId")]
    tmdb_id: Option<i64>,
    status: Option<i32>,
}

impl HomelabClient {
    pub fn new(config: Config) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .pool_max_idle_per_host(1)
            .pool_idle_timeout(Duration::from_secs(15))
            .tcp_nodelay(true)
            .user_agent("BollySeerr/0.1.1")
            .build()
            .expect("failed to build reqwest client");
        Self { client, config }
    }

    pub async fn sync_all_statuses(&self, db: &Database) -> Result<usize> {
        let mut movie_states: HashMap<i64, (HomelabStatus, Option<i64>, Option<i64>)> =
            HashMap::new();
        let mut tv_states: HashMap<i64, (HomelabStatus, Option<i64>, Option<i64>)> = HashMap::new();

        // 1. Check Seerr requests first
        if let Some(seerr_key) = self.config.resolve_seerr_api_key() {
            match self.fetch_seerr_requests(&seerr_key).await {
                Ok(reqs) => {
                    for r in reqs {
                        if let Some(media) = r.media
                            && let Some(tmdb_id) = media.tmdb_id
                        {
                            // Seerr media.status: 5 = Available, 4 = Partially Available, 3 = Processing, 2 = Pending
                            let st = match media.status.unwrap_or(2) {
                                5 => HomelabStatus::Available,
                                4 => HomelabStatus::PartiallyAvailable,
                                3 => HomelabStatus::Monitored,
                                _ => HomelabStatus::Requested,
                            };
                            if r.media_type == "tv" {
                                tv_states.insert(tmdb_id, (st, None, Some(r.id)));
                            } else {
                                movie_states.insert(tmdb_id, (st, None, Some(r.id)));
                            }
                        }
                    }
                }
                Err(e) => debug!("Seerr status check skipped/failed: {e:#}"),
            }
        }

        // 2. Check Radarr movies (higher fidelity for movies)
        if let Some(ref radarr_key) = self.config.radarr_api_key {
            match self.fetch_radarr_movies(radarr_key).await {
                Ok(movies) => {
                    for m in movies {
                        let st = if m.has_file {
                            HomelabStatus::Available
                        } else if m.monitored {
                            HomelabStatus::Monitored
                        } else {
                            HomelabStatus::Requested
                        };
                        let existing_seerr = movie_states.get(&m.tmdb_id).and_then(|v| v.2);
                        movie_states.insert(m.tmdb_id, (st, Some(m.id), existing_seerr));
                    }
                }
                Err(e) => debug!("Radarr status check skipped/failed: {e:#}"),
            }
        }

        // 3. Check Sonarr series (higher fidelity for web series)
        if let Some(ref sonarr_key) = self.config.sonarr_api_key {
            match self.fetch_sonarr_series(sonarr_key).await {
                Ok(series_list) => {
                    for s in series_list {
                        if let Some(tmdb_id) = s.tmdb_id {
                            let st = if let Some(stats) = s.statistics {
                                if stats.episode_count > 0
                                    && stats.episode_file_count >= stats.episode_count
                                {
                                    HomelabStatus::Available
                                } else if stats.episode_file_count > 0 {
                                    HomelabStatus::PartiallyAvailable
                                } else if s.monitored {
                                    HomelabStatus::Monitored
                                } else {
                                    HomelabStatus::Requested
                                }
                            } else if s.monitored {
                                HomelabStatus::Monitored
                            } else {
                                HomelabStatus::Requested
                            };
                            let existing_seerr = tv_states.get(&tmdb_id).and_then(|v| v.2);
                            tv_states.insert(tmdb_id, (st, Some(s.id), existing_seerr));
                        }
                    }
                }
                Err(e) => debug!("Sonarr status check skipped/failed: {e:#}"),
            }
        }

        // Apply statuses to all items in local SQLite DB
        let items = db.get_all_items()?;
        let mut updated = 0;

        for item in items {
            match item.media_type {
                MediaType::Movie => {
                    if let Some((status, radarr_id, seerr_id)) = movie_states.get(&item.tmdb_id)
                        && (item.homelab_status != *status
                            || item.radarr_id != *radarr_id
                            || item.seerr_request_id != *seerr_id)
                    {
                        db.update_homelab_status(
                            item.tmdb_id,
                            MediaType::Movie,
                            *status,
                            *radarr_id,
                            None,
                            *seerr_id,
                        )?;
                        updated += 1;
                    }
                }
                MediaType::Tv => {
                    if let Some((status, sonarr_id, seerr_id)) = tv_states.get(&item.tmdb_id)
                        && (item.homelab_status != *status
                            || item.sonarr_id != *sonarr_id
                            || item.seerr_request_id != *seerr_id)
                    {
                        db.update_homelab_status(
                            item.tmdb_id,
                            MediaType::Tv,
                            *status,
                            None,
                            *sonarr_id,
                            *seerr_id,
                        )?;
                        updated += 1;
                    }
                }
            }
        }

        Ok(updated)
    }

    pub async fn request_media(
        &self,
        db: &Database,
        tmdb_id: i64,
        media_type: MediaType,
        seasons: Option<String>,
    ) -> Result<RequestResult> {
        // 1. Primary path: Request via Seerr API
        if let Some(seerr_key) = self.config.resolve_seerr_api_key() {
            match self
                .request_via_seerr(&seerr_key, tmdb_id, media_type, seasons.clone())
                .await
            {
                Ok(seerr_req_id) => {
                    info!(
                        "Successfully requested {:?} (TMDB {}) via Seerr (request_id={:?})",
                        media_type, tmdb_id, seerr_req_id
                    );
                    db.update_homelab_status(
                        tmdb_id,
                        media_type,
                        HomelabStatus::Requested,
                        None,
                        None,
                        seerr_req_id,
                    )?;
                    return Ok(RequestResult {
                        success: true,
                        status: HomelabStatus::Requested,
                        message: "Requested via Jellyseerr (Seerr)".to_string(),
                        via: "seerr".to_string(),
                    });
                }
                Err(e) => {
                    warn!(
                        "Seerr request failed for {:?} TMDB {}, falling back to direct Radarr/Sonarr: {e:#}",
                        media_type, tmdb_id
                    );
                }
            }
        }

        // 2. Fallback path: Direct to Radarr (for Movie) or Sonarr (for TV)
        match media_type {
            MediaType::Movie => {
                let radarr_key = self
                    .config
                    .radarr_api_key
                    .as_ref()
                    .ok_or_else(|| anyhow!("Neither Seerr nor Radarr API key is configured"))?;
                let radarr_id = self.request_via_radarr(radarr_key, tmdb_id).await?;
                db.update_homelab_status(
                    tmdb_id,
                    MediaType::Movie,
                    HomelabStatus::Monitored,
                    Some(radarr_id),
                    None,
                    None,
                )?;
                Ok(RequestResult {
                    success: true,
                    status: HomelabStatus::Monitored,
                    message: "Added and monitored directly in Radarr".to_string(),
                    via: "radarr".to_string(),
                })
            }
            MediaType::Tv => {
                let sonarr_key = self
                    .config
                    .sonarr_api_key
                    .as_ref()
                    .ok_or_else(|| anyhow!("Neither Seerr nor Sonarr API key is configured"))?;
                let sonarr_id = self.request_via_sonarr(sonarr_key, tmdb_id).await?;
                db.update_homelab_status(
                    tmdb_id,
                    MediaType::Tv,
                    HomelabStatus::Monitored,
                    None,
                    Some(sonarr_id),
                    None,
                )?;
                Ok(RequestResult {
                    success: true,
                    status: HomelabStatus::Monitored,
                    message: "Added and monitored directly in Sonarr".to_string(),
                    via: "sonarr".to_string(),
                })
            }
        }
    }

    pub async fn delete_media(
        &self,
        db: &Database,
        tmdb_id: i64,
        media_type: MediaType,
    ) -> Result<DeleteResult> {
        let item = db
            .get_item(tmdb_id, media_type)?
            .ok_or_else(|| anyhow!("Item not found in catalog"))?;

        let mut radarr_deleted = false;
        let mut sonarr_deleted = false;
        let mut seerr_deleted = false;
        let mut jellyfin_refreshed = false;

        // 1. Delete from Radarr (if movie)
        if media_type == MediaType::Movie
            && let Some(ref radarr_key) = self.config.radarr_api_key
        {
            let radarr_id = match item.radarr_id {
                Some(id) => Some(id),
                None => {
                    let movies = self
                        .fetch_radarr_movies(radarr_key)
                        .await
                        .unwrap_or_default();
                    movies
                        .into_iter()
                        .find(|m| m.tmdb_id == tmdb_id)
                        .map(|m| m.id)
                }
            };

            if let Some(id) = radarr_id {
                match self.delete_radarr_movie(radarr_key, id).await {
                    Ok(()) => {
                        info!(
                            "Successfully deleted movie {tmdb_id} (radarr_id {id}) from Radarr with files"
                        );
                        radarr_deleted = true;
                    }
                    Err(e) => {
                        warn!("Failed to delete movie {tmdb_id} from Radarr: {e:#}");
                    }
                }
            }
        }

        // 2. Delete from Sonarr (if tv)
        if media_type == MediaType::Tv
            && let Some(ref sonarr_key) = self.config.sonarr_api_key
        {
            let sonarr_id = match item.sonarr_id {
                Some(id) => Some(id),
                None => {
                    let series = self
                        .fetch_sonarr_series(sonarr_key)
                        .await
                        .unwrap_or_default();
                    series
                        .into_iter()
                        .find(|s| s.tmdb_id == Some(tmdb_id))
                        .map(|s| s.id)
                }
            };

            if let Some(id) = sonarr_id {
                match self.delete_sonarr_series(sonarr_key, id).await {
                    Ok(()) => {
                        info!(
                            "Successfully deleted series {tmdb_id} (sonarr_id {id}) from Sonarr with files"
                        );
                        sonarr_deleted = true;
                    }
                    Err(e) => {
                        warn!("Failed to delete series {tmdb_id} from Sonarr: {e:#}");
                    }
                }
            }
        }

        // 3. Delete from Seerr (request and media item)
        if let Some(seerr_key) = self.config.resolve_seerr_api_key() {
            // Delete request if known
            if let Some(req_id) = item.seerr_request_id {
                let _ = self.delete_seerr_request(&seerr_key, req_id).await;
                seerr_deleted = true;
            }

            // Find requests matching this tmdb_id and delete them
            if let Ok(reqs) = self.fetch_seerr_requests(&seerr_key).await {
                for r in reqs {
                    if let Some(media) = r.media
                        && media.tmdb_id == Some(tmdb_id)
                    {
                        let _ = self.delete_seerr_request(&seerr_key, r.id).await;
                        seerr_deleted = true;
                    }
                }
            }

            // Delete media item in Seerr
            match self
                .fetch_and_delete_seerr_media(&seerr_key, tmdb_id, media_type)
                .await
            {
                Ok(deleted) => {
                    if deleted {
                        seerr_deleted = true;
                    }
                }
                Err(e) => {
                    debug!("Seerr media removal skipped/failed: {e:#}");
                }
            }
        }

        // 4. Jellyfin library refresh (if configured)
        if self.config.jellyfin_url.is_some() {
            match self.refresh_jellyfin_library().await {
                Ok(()) => {
                    jellyfin_refreshed = true;
                }
                Err(e) => {
                    warn!("Jellyfin library refresh failed: {e:#}");
                }
            }
        }

        // 5. Reset local DB status to unrequested
        db.reset_homelab_status(tmdb_id, media_type)?;
        info!(
            "Reset homelab status for {:?} tmdb_id {}",
            media_type, tmdb_id
        );

        let mut actions = Vec::new();
        if radarr_deleted {
            actions.push("Radarr (media files deleted)");
        }
        if sonarr_deleted {
            actions.push("Sonarr (media files deleted)");
        }
        if seerr_deleted {
            actions.push("Jellyseerr");
        }
        if jellyfin_refreshed {
            actions.push("Jellyfin library refreshed");
        } else {
            actions.push("Jellyfin library updated via file removal");
        }

        let message = format!("Successfully removed from {}", actions.join(", "));

        Ok(DeleteResult {
            success: true,
            message,
            radarr_deleted,
            sonarr_deleted,
            seerr_deleted,
            jellyfin_refreshed,
        })
    }

    async fn delete_radarr_movie(&self, api_key: &str, radarr_id: i64) -> Result<()> {
        let url = format!(
            "{}/api/v3/movie/{radarr_id}?deleteFiles=true&addImportExclusion=false",
            self.config.radarr_url
        );
        let resp = self
            .client
            .delete(&url)
            .header("X-Api-Key", api_key)
            .send()
            .await
            .context("Failed to send DELETE to Radarr")?;

        if resp.status().is_success() || resp.status() == reqwest::StatusCode::NOT_FOUND {
            Ok(())
        } else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            Err(anyhow!("Radarr delete failed ({status}): {body}"))
        }
    }

    async fn delete_sonarr_series(&self, api_key: &str, sonarr_id: i64) -> Result<()> {
        let url = format!(
            "{}/api/v3/series/{sonarr_id}?deleteFiles=true&addImportListExclusion=false",
            self.config.sonarr_url
        );
        let resp = self
            .client
            .delete(&url)
            .header("X-Api-Key", api_key)
            .send()
            .await
            .context("Failed to send DELETE to Sonarr")?;

        if resp.status().is_success() || resp.status() == reqwest::StatusCode::NOT_FOUND {
            Ok(())
        } else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            Err(anyhow!("Sonarr delete failed ({status}): {body}"))
        }
    }

    async fn delete_seerr_request(&self, api_key: &str, request_id: i64) -> Result<()> {
        let url = format!("{}/api/v1/request/{request_id}", self.config.seerr_url);
        let resp = self
            .client
            .delete(&url)
            .header("X-Api-Key", api_key)
            .send()
            .await?;

        if resp.status().is_success() || resp.status() == reqwest::StatusCode::NOT_FOUND {
            Ok(())
        } else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            Err(anyhow!("Seerr delete request failed ({status}): {body}"))
        }
    }

    async fn fetch_and_delete_seerr_media(
        &self,
        api_key: &str,
        tmdb_id: i64,
        media_type: MediaType,
    ) -> Result<bool> {
        let endpoint = match media_type {
            MediaType::Movie => "movie",
            MediaType::Tv => "tv",
        };
        let get_url = format!("{}/api/v1/{endpoint}/{tmdb_id}", self.config.seerr_url);
        let resp = self
            .client
            .get(&get_url)
            .header("X-Api-Key", api_key)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Ok(false);
        }

        let val: serde_json::Value = resp.json().await.unwrap_or_default();
        let media_id = val
            .get("mediaInfo")
            .and_then(|m| m.get("id"))
            .and_then(serde_json::Value::as_i64);

        if let Some(m_id) = media_id {
            let del_url = format!("{}/api/v1/media/{m_id}", self.config.seerr_url);
            let del_resp = self
                .client
                .delete(&del_url)
                .header("X-Api-Key", api_key)
                .send()
                .await?;

            if del_resp.status().is_success() || del_resp.status() == reqwest::StatusCode::NOT_FOUND
            {
                info!("Deleted Seerr mediaInfo {m_id} for {endpoint} tmdb_id {tmdb_id}");
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn refresh_jellyfin_library(&self) -> Result<()> {
        let jellyfin_url = match self.config.jellyfin_url {
            Some(ref u) => u,
            None => return Ok(()),
        };

        let url = format!("{jellyfin_url}/Library/Refresh");
        let mut req = self.client.post(&url);

        if let Some(ref key) = self.config.jellyfin_api_key {
            req = req
                .header("X-Emby-Token", key)
                .header("Authorization", format!("MediaBrowser Token=\"{key}\""));
        }

        let resp = req.send().await.context("Failed to connect to Jellyfin")?;
        if resp.status().is_success() {
            info!("Successfully refreshed Jellyfin library");
            Ok(())
        } else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            Err(anyhow!("Jellyfin refresh returned HTTP {status}: {body}"))
        }
    }

    async fn fetch_seerr_requests(&self, api_key: &str) -> Result<Vec<SeerrRequestItem>> {
        let url = format!(
            "{}/api/v1/request?take=200&skip=0&filter=all",
            self.config.seerr_url
        );
        let resp = self
            .client
            .get(&url)
            .header("X-Api-Key", api_key)
            .send()
            .await
            .context("Failed to connect to Seerr")?;

        if !resp.status().is_success() {
            return Err(anyhow!("Seerr returned HTTP {}", resp.status()));
        }

        let list: SeerrRequestList = resp.json().await?;
        Ok(list.results)
    }

    async fn request_via_seerr(
        &self,
        api_key: &str,
        tmdb_id: i64,
        media_type: MediaType,
        seasons: Option<String>,
    ) -> Result<Option<i64>> {
        let url = format!("{}/api/v1/request", self.config.seerr_url);
        let body = match media_type {
            MediaType::Movie => json!({
                "mediaType": "movie",
                "mediaId": tmdb_id,
            }),
            MediaType::Tv => {
                // In Seerr API v1, requesting all seasons accepts "seasons": "all"
                let s = seasons.unwrap_or_else(|| "all".to_string());
                json!({
                    "mediaType": "tv",
                    "mediaId": tmdb_id,
                    "seasons": s,
                })
            }
        };

        let resp = self
            .client
            .post(&url)
            .header("X-Api-Key", api_key)
            .json(&body)
            .send()
            .await
            .context("Failed to send POST /api/v1/request to Seerr")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err_text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Seerr request failed ({status}): {err_text}"));
        }

        let val: serde_json::Value = resp.json().await.unwrap_or_default();
        let req_id = val.get("id").and_then(serde_json::Value::as_i64);
        Ok(req_id)
    }

    async fn fetch_radarr_movies(&self, api_key: &str) -> Result<Vec<RadarrMovie>> {
        let url = format!("{}/api/v3/movie", self.config.radarr_url);
        let resp = self
            .client
            .get(&url)
            .header("X-Api-Key", api_key)
            .send()
            .await
            .context("Failed to connect to Radarr")?;

        if !resp.status().is_success() {
            return Err(anyhow!("Radarr returned HTTP {}", resp.status()));
        }

        let movies: Vec<RadarrMovie> = resp.json().await?;
        Ok(movies)
    }

    async fn request_via_radarr(&self, api_key: &str, tmdb_id: i64) -> Result<i64> {
        // Check if already in Radarr
        let existing = self.fetch_radarr_movies(api_key).await?;
        if let Some(found) = existing.into_iter().find(|m| m.tmdb_id == tmdb_id) {
            return Ok(found.id);
        }

        // Lookup movie from Radarr's TMDB lookup endpoint
        let lookup_url = format!(
            "{}/api/v3/movie/lookup/tmdb?tmdbId={tmdb_id}",
            self.config.radarr_url
        );
        let lookup_resp = self
            .client
            .get(&lookup_url)
            .header("X-Api-Key", api_key)
            .send()
            .await?;

        if !lookup_resp.status().is_success() {
            return Err(anyhow!(
                "Radarr lookup failed with HTTP {}",
                lookup_resp.status()
            ));
        }

        let mut movie_json: serde_json::Value = lookup_resp.json().await?;

        // Get root folder and quality profile
        let root_folders: Vec<RadarrRootFolder> = self
            .client
            .get(format!("{}/api/v3/rootfolder", self.config.radarr_url))
            .header("X-Api-Key", api_key)
            .send()
            .await?
            .json()
            .await?;

        let root_path = root_folders
            .first()
            .map(|r| r.path.clone())
            .unwrap_or_else(|| "/media/movies".to_string());

        let profiles: Vec<RadarrQualityProfile> = self
            .client
            .get(format!("{}/api/v3/qualityprofile", self.config.radarr_url))
            .header("X-Api-Key", api_key)
            .send()
            .await?
            .json()
            .await?;

        let profile_id = profiles.first().map(|p| p.id).unwrap_or(1);

        if let Some(obj) = movie_json.as_object_mut() {
            obj.insert("qualityProfileId".to_string(), json!(profile_id));
            obj.insert("rootFolderPath".to_string(), json!(root_path));
            obj.insert("monitored".to_string(), json!(true));
            obj.insert("addOptions".to_string(), json!({ "searchForMovie": true }));
        }

        let add_resp = self
            .client
            .post(format!("{}/api/v3/movie", self.config.radarr_url))
            .header("X-Api-Key", api_key)
            .json(&movie_json)
            .send()
            .await?;

        if !add_resp.status().is_success() {
            let err_body = add_resp.text().await.unwrap_or_default();
            return Err(anyhow!("Failed to add movie to Radarr: {err_body}"));
        }

        let created: serde_json::Value = add_resp.json().await?;
        Ok(created
            .get("id")
            .and_then(serde_json::Value::as_i64)
            .unwrap_or(0))
    }

    async fn fetch_sonarr_series(&self, api_key: &str) -> Result<Vec<SonarrSeries>> {
        let url = format!("{}/api/v3/series", self.config.sonarr_url);
        let resp = self
            .client
            .get(&url)
            .header("X-Api-Key", api_key)
            .send()
            .await
            .context("Failed to connect to Sonarr")?;

        if !resp.status().is_success() {
            return Err(anyhow!("Sonarr returned HTTP {}", resp.status()));
        }

        let series: Vec<SonarrSeries> = resp.json().await?;
        Ok(series)
    }

    async fn request_via_sonarr(&self, api_key: &str, tmdb_id: i64) -> Result<i64> {
        let existing = self.fetch_sonarr_series(api_key).await?;
        if let Some(found) = existing.into_iter().find(|s| s.tmdb_id == Some(tmdb_id)) {
            return Ok(found.id);
        }

        let lookup_url = format!(
            "{}/api/v3/series/lookup?term=tmdb:{tmdb_id}",
            self.config.sonarr_url
        );
        let lookup_resp = self
            .client
            .get(&lookup_url)
            .header("X-Api-Key", api_key)
            .send()
            .await?;

        if !lookup_resp.status().is_success() {
            return Err(anyhow!(
                "Sonarr lookup failed with HTTP {}",
                lookup_resp.status()
            ));
        }

        let mut results: Vec<serde_json::Value> = lookup_resp.json().await?;
        let mut series_json = results
            .drain(..)
            .next()
            .ok_or_else(|| anyhow!("No series found in Sonarr lookup for tmdb:{tmdb_id}"))?;

        let root_folders: Vec<RadarrRootFolder> = self
            .client
            .get(format!("{}/api/v3/rootfolder", self.config.sonarr_url))
            .header("X-Api-Key", api_key)
            .send()
            .await?
            .json()
            .await?;

        let root_path = root_folders
            .first()
            .map(|r| r.path.clone())
            .unwrap_or_else(|| "/media/tv".to_string());

        let profiles: Vec<RadarrQualityProfile> = self
            .client
            .get(format!("{}/api/v3/qualityprofile", self.config.sonarr_url))
            .header("X-Api-Key", api_key)
            .send()
            .await?
            .json()
            .await?;

        let profile_id = profiles.first().map(|p| p.id).unwrap_or(1);

        if let Some(obj) = series_json.as_object_mut() {
            obj.insert("qualityProfileId".to_string(), json!(profile_id));
            obj.insert("rootFolderPath".to_string(), json!(root_path));
            obj.insert("monitored".to_string(), json!(true));
            obj.insert("seasonFolder".to_string(), json!(true));
            obj.insert(
                "addOptions".to_string(),
                json!({
                    "monitor": "all",
                    "searchForMissingEpisodes": true,
                    "searchForCutoffUnmetEpisodes": false
                }),
            );
        }

        let add_resp = self
            .client
            .post(format!("{}/api/v3/series", self.config.sonarr_url))
            .header("X-Api-Key", api_key)
            .json(&series_json)
            .send()
            .await?;

        if !add_resp.status().is_success() {
            let err_body = add_resp.text().await.unwrap_or_default();
            return Err(anyhow!("Failed to add series to Sonarr: {err_body}"));
        }

        let created: serde_json::Value = add_resp.json().await?;
        Ok(created
            .get("id")
            .and_then(serde_json::Value::as_i64)
            .unwrap_or(0))
    }
}
