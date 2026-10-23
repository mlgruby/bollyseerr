use crate::models::{HomelabStatus, MediaItem, MediaType, SyncStatus};
use anyhow::{Context, Result};
use chrono::Utc;
use rusqlite::{Connection, params};
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn open(path: &str) -> Result<Self> {
        if path != ":memory:"
            && let Some(parent) = Path::new(path).parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create DB directory {:?}", parent))?;
        }

        let conn = Connection::open(path)
            .with_context(|| format!("Failed to open SQLite database at {path}"))?;

        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;",
        )?;

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&self) -> Result<()> {
        let conn = self.conn.lock().expect("db mutex poisoned");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS media_items (
                tmdb_id INTEGER NOT NULL,
                media_type TEXT NOT NULL,
                title TEXT NOT NULL,
                original_title TEXT NOT NULL,
                original_language TEXT NOT NULL,
                is_hindi_dub INTEGER NOT NULL DEFAULT 0,
                overview TEXT NOT NULL DEFAULT '',
                poster_path TEXT,
                backdrop_path TEXT,
                release_date TEXT NOT NULL,
                vote_average REAL NOT NULL DEFAULT 0.0,
                vote_count INTEGER NOT NULL DEFAULT 0,
                popularity REAL NOT NULL DEFAULT 0.0,
                genres_json TEXT NOT NULL DEFAULT '[]',
                ott_providers_json TEXT NOT NULL DEFAULT '[]',
                release_type TEXT NOT NULL DEFAULT 'Theatrical',
                trailer_key TEXT,
                season_count INTEGER,
                latest_season_number INTEGER,
                homelab_status TEXT NOT NULL DEFAULT 'unrequested',
                radarr_id INTEGER,
                sonarr_id INTEGER,
                seerr_request_id INTEGER,
                updated_at TEXT NOT NULL,
                PRIMARY KEY (tmdb_id, media_type)
            );

            CREATE INDEX IF NOT EXISTS idx_media_release_date ON media_items(release_date);
            CREATE INDEX IF NOT EXISTS idx_media_type ON media_items(media_type);
            CREATE INDEX IF NOT EXISTS idx_homelab_status ON media_items(homelab_status);

            CREATE TABLE IF NOT EXISTS sync_meta (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );",
        )?;
        Ok(())
    }

    pub fn upsert_item(&self, item: &MediaItem) -> Result<()> {
        let conn = self.conn.lock().expect("db mutex poisoned");
        let genres_json = serde_json::to_string(&item.genres).unwrap_or_else(|_| "[]".to_string());
        let ott_json =
            serde_json::to_string(&item.ott_providers).unwrap_or_else(|_| "[]".to_string());
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO media_items (
                tmdb_id, media_type, title, original_title, original_language,
                is_hindi_dub, overview, poster_path, backdrop_path, release_date,
                vote_average, vote_count, popularity, genres_json, ott_providers_json,
                release_type, trailer_key, season_count, latest_season_number,
                homelab_status, radarr_id, sonarr_id, seerr_request_id, updated_at
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5,
                ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15,
                ?16, ?17, ?18, ?19,
                ?20, ?21, ?22, ?23, ?24
            )
            ON CONFLICT(tmdb_id, media_type) DO UPDATE SET
                title = excluded.title,
                original_title = excluded.original_title,
                original_language = excluded.original_language,
                is_hindi_dub = excluded.is_hindi_dub,
                overview = CASE WHEN excluded.overview != '' THEN excluded.overview ELSE media_items.overview END,
                poster_path = COALESCE(excluded.poster_path, media_items.poster_path),
                backdrop_path = COALESCE(excluded.backdrop_path, media_items.backdrop_path),
                release_date = excluded.release_date,
                vote_average = excluded.vote_average,
                vote_count = excluded.vote_count,
                popularity = excluded.popularity,
                genres_json = CASE WHEN excluded.genres_json != '[]' THEN excluded.genres_json ELSE media_items.genres_json END,
                ott_providers_json = CASE WHEN excluded.ott_providers_json != '[]' THEN excluded.ott_providers_json ELSE media_items.ott_providers_json END,
                release_type = excluded.release_type,
                trailer_key = COALESCE(excluded.trailer_key, media_items.trailer_key),
                season_count = COALESCE(excluded.season_count, media_items.season_count),
                latest_season_number = COALESCE(excluded.latest_season_number, media_items.latest_season_number),
                updated_at = excluded.updated_at",
            params![
                item.tmdb_id,
                item.media_type.as_str(),
                item.title,
                item.original_title,
                item.original_language,
                i32::from(item.is_hindi_dub),
                item.overview,
                item.poster_path,
                item.backdrop_path,
                item.release_date,
                item.vote_average,
                item.vote_count,
                item.popularity,
                genres_json,
                ott_json,
                item.release_type,
                item.trailer_key,
                item.season_count,
                item.latest_season_number,
                item.homelab_status.as_str(),
                item.radarr_id,
                item.sonarr_id,
                item.seerr_request_id,
                now,
            ],
        )?;
        Ok(())
    }

    pub fn prune_older_than(&self, cutoff_date: &str) -> Result<usize> {
        let conn = self.conn.lock().expect("db mutex poisoned");
        let deleted = conn.execute(
            "DELETE FROM media_items WHERE release_date < ?1",
            params![cutoff_date],
        )?;
        Ok(deleted)
    }

    pub fn update_homelab_status(
        &self,
        tmdb_id: i64,
        media_type: MediaType,
        status: HomelabStatus,
        radarr_id: Option<i64>,
        sonarr_id: Option<i64>,
        seerr_request_id: Option<i64>,
    ) -> Result<()> {
        let conn = self.conn.lock().expect("db mutex poisoned");
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE media_items
             SET homelab_status = ?1,
                 radarr_id = COALESCE(?2, radarr_id),
                 sonarr_id = COALESCE(?3, sonarr_id),
                 seerr_request_id = COALESCE(?4, seerr_request_id),
                 updated_at = ?5
             WHERE tmdb_id = ?6 AND media_type = ?7",
            params![
                status.as_str(),
                radarr_id,
                sonarr_id,
                seerr_request_id,
                now,
                tmdb_id,
                media_type.as_str()
            ],
        )?;
        Ok(())
    }

    pub fn get_all_items(&self) -> Result<Vec<MediaItem>> {
        let conn = self.conn.lock().expect("db mutex poisoned");
        let mut stmt = conn.prepare(
            "SELECT
                tmdb_id, media_type, title, original_title, original_language,
                is_hindi_dub, overview, poster_path, backdrop_path, release_date,
                vote_average, vote_count, popularity, genres_json, ott_providers_json,
                release_type, trailer_key, season_count, latest_season_number,
                homelab_status, radarr_id, sonarr_id, seerr_request_id, updated_at
             FROM media_items
             ORDER BY release_date DESC, popularity DESC",
        )?;

        let rows = stmt.query_map([], |row| {
            let media_type_str: String = row.get(1)?;
            let is_hindi_dub_int: i32 = row.get(5)?;
            let genres_str: String = row.get(13)?;
            let ott_str: String = row.get(14)?;
            let status_str: String = row.get(19)?;

            Ok(MediaItem {
                tmdb_id: row.get(0)?,
                media_type: MediaType::parse_str(&media_type_str),
                title: row.get(2)?,
                original_title: row.get(3)?,
                original_language: row.get(4)?,
                is_hindi_dub: is_hindi_dub_int != 0,
                overview: row.get(6)?,
                poster_path: row.get(7)?,
                backdrop_path: row.get(8)?,
                release_date: row.get(9)?,
                vote_average: row.get(10)?,
                vote_count: row.get(11)?,
                popularity: row.get(12)?,
                genres: serde_json::from_str(&genres_str).unwrap_or_default(),
                ott_providers: serde_json::from_str(&ott_str).unwrap_or_default(),
                release_type: row.get(15)?,
                trailer_key: row.get(16)?,
                season_count: row.get(17)?,
                latest_season_number: row.get(18)?,
                homelab_status: HomelabStatus::parse_str(&status_str),
                radarr_id: row.get(20)?,
                sonarr_id: row.get(21)?,
                seerr_request_id: row.get(22)?,
                updated_at: row.get(23)?,
            })
        })?;

        let mut items = Vec::new();
        for item in rows {
            items.push(item?);
        }
        Ok(items)
    }

    pub fn get_item(&self, tmdb_id: i64, media_type: MediaType) -> Result<Option<MediaItem>> {
        let all = self.get_all_items()?;
        Ok(all
            .into_iter()
            .find(|i| i.tmdb_id == tmdb_id && i.media_type == media_type))
    }

    pub fn set_meta(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.conn.lock().expect("db mutex poisoned");
        conn.execute(
            "INSERT INTO sync_meta (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn get_meta(&self, key: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().expect("db mutex poisoned");
        let mut stmt = conn.prepare("SELECT value FROM sync_meta WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn get_sync_status(&self, today_date: &str, is_syncing: bool) -> Result<SyncStatus> {
        let items = self.get_all_items()?;
        let total_movies = items
            .iter()
            .filter(|i| i.media_type == MediaType::Movie)
            .count();
        let total_series = items
            .iter()
            .filter(|i| i.media_type == MediaType::Tv)
            .count();
        let released_count = items
            .iter()
            .filter(|i| i.release_date.as_str() <= today_date)
            .count();
        let upcoming_count = items
            .iter()
            .filter(|i| i.release_date.as_str() > today_date)
            .count();
        let library_or_requested_count = items
            .iter()
            .filter(|i| i.homelab_status != HomelabStatus::Unrequested)
            .count();

        let last_discovery_sync = self.get_meta("last_discovery_sync")?;
        let last_status_sync = self.get_meta("last_status_sync")?;
        let pruned_last_run = self
            .get_meta("pruned_last_run")?
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        Ok(SyncStatus {
            last_discovery_sync,
            last_status_sync,
            total_movies,
            total_series,
            released_count,
            upcoming_count,
            library_or_requested_count,
            pruned_last_run,
            is_syncing,
        })
    }
}
