use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub database_path: String,
    pub tmdb_api_key: String,
    pub tmdb_base_url: String,
    pub seerr_url: String,
    pub seerr_api_key: Option<String>,
    pub seerr_settings_path: String,
    pub radarr_url: String,
    pub radarr_api_key: Option<String>,
    pub sonarr_url: String,
    pub sonarr_api_key: Option<String>,
    pub retention_days: i64,
    pub upcoming_days: i64,
    pub discovery_interval_seconds: u64,
    pub status_sync_interval_seconds: u64,
    pub max_pages_per_query: u32,
}

impl Config {
    pub fn from_env() -> Self {
        let port = env::var("PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5056);

        let database_path =
            env::var("DATABASE_PATH").unwrap_or_else(|_| "/data/bollyseerr.db".to_string());

        // Default to Seerr's standard embedded TMDB v3 key if user did not override
        let tmdb_api_key = env::var("TMDB_API_KEY")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| "431a8708161bcd1f1fbe7536137e61ed".to_string());

        let tmdb_base_url = env::var("TMDB_BASE_URL")
            .unwrap_or_else(|_| "https://api.themoviedb.org/3".to_string());

        let seerr_url =
            env::var("SEERR_URL").unwrap_or_else(|_| "http://172.39.0.9:5055".to_string());

        let seerr_settings_path = env::var("SEERR_SETTINGS_PATH")
            .unwrap_or_else(|_| "/seerr-config/settings.json".to_string());

        let mut seerr_api_key = env::var("SEERR_API_KEY")
            .ok()
            .filter(|s| !s.trim().is_empty());

        if seerr_api_key.is_none() {
            seerr_api_key = Self::read_seerr_api_key_from_settings(&seerr_settings_path);
        }

        let radarr_url =
            env::var("RADARR_URL").unwrap_or_else(|_| "http://172.39.0.4:7878".to_string());

        let radarr_api_key = env::var("RADARR_API_KEY")
            .or_else(|_| env::var("SERVARR_RADARR_API_KEY"))
            .ok()
            .filter(|s| !s.trim().is_empty());

        let sonarr_url =
            env::var("SONARR_URL").unwrap_or_else(|_| "http://172.39.0.3:8989".to_string());

        let sonarr_api_key = env::var("SONARR_API_KEY")
            .or_else(|_| env::var("SERVARR_SONARR_API_KEY"))
            .ok()
            .filter(|s| !s.trim().is_empty());

        let retention_days = env::var("RETENTION_DAYS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(150);

        let upcoming_days = env::var("UPCOMING_DAYS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(180);

        let discovery_interval_seconds = env::var("DISCOVERY_INTERVAL_SECONDS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(21_600); // 6 hours

        let status_sync_interval_seconds = env::var("STATUS_SYNC_INTERVAL_SECONDS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(300); // 5 minutes

        let max_pages_per_query = env::var("MAX_PAGES_PER_QUERY")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(15);

        Self {
            port,
            database_path,
            tmdb_api_key,
            tmdb_base_url,
            seerr_url: seerr_url.trim_end_matches('/').to_string(),
            seerr_api_key,
            seerr_settings_path,
            radarr_url: radarr_url.trim_end_matches('/').to_string(),
            radarr_api_key,
            sonarr_url: sonarr_url.trim_end_matches('/').to_string(),
            sonarr_api_key,
            retention_days,
            upcoming_days,
            discovery_interval_seconds,
            status_sync_interval_seconds,
            max_pages_per_query,
        }
    }

    pub fn resolve_seerr_api_key(&self) -> Option<String> {
        if let Some(ref key) = self.seerr_api_key {
            return Some(key.clone());
        }
        Self::read_seerr_api_key_from_settings(&self.seerr_settings_path)
    }

    pub fn read_seerr_api_key_from_settings(path: &str) -> Option<String> {
        let p = Path::new(path);
        if !p.exists() {
            return None;
        }
        let content = fs::read_to_string(p).ok()?;
        let json: serde_json::Value = serde_json::from_str(&content).ok()?;
        json.get("main")?
            .get("apiKey")?
            .as_str()
            .map(std::string::ToString::to_string)
            .filter(|s| !s.trim().is_empty())
    }
}
