use crate::config::Config;
use crate::db::Database;
use crate::homelab::HomelabClient;
use crate::tmdb::TmdbClient;
use anyhow::Result;
use chrono::{Duration as ChronoDuration, Utc};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tracing::{error, info};

#[derive(Clone)]
pub struct AgentState {
    pub config: Config,
    pub db: Database,
    pub tmdb: Arc<TmdbClient>,
    pub homelab: Arc<HomelabClient>,
    pub is_syncing: Arc<AtomicBool>,
}

impl AgentState {
    pub fn new(config: Config, db: Database) -> Self {
        let tmdb = Arc::new(TmdbClient::new(
            config.tmdb_api_key.clone(),
            config.tmdb_base_url.clone(),
            config.max_pages_per_query,
        ));
        let homelab = Arc::new(HomelabClient::new(config.clone()));
        Self {
            config,
            db,
            tmdb,
            homelab,
            is_syncing: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn spawn_background_loops(&self) {
        let state_discovery = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(
                state_discovery.config.discovery_interval_seconds,
            ));
            loop {
                interval.tick().await;
                if let Err(e) = state_discovery.run_discovery_and_prune().await {
                    error!("Discovery & prune loop error: {e:#}");
                }
            }
        });

        let state_status = self.clone();
        tokio::spawn(async move {
            // Wait 15s initially so startup discovery can begin populating
            tokio::time::sleep(Duration::from_secs(15)).await;
            let mut interval = tokio::time::interval(Duration::from_secs(
                state_status.config.status_sync_interval_seconds,
            ));
            loop {
                interval.tick().await;
                if let Err(e) = state_status.run_status_sync().await {
                    error!("Homelab status sync loop error: {e:#}");
                }
            }
        });
    }

    pub async fn run_discovery_and_prune(&self) -> Result<()> {
        if self
            .is_syncing
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            info!("Discovery sync already in progress, skipping duplicate trigger");
            return Ok(());
        }

        let res = self.inner_discovery_and_prune().await;
        self.is_syncing.store(false, Ordering::SeqCst);
        res
    }

    async fn inner_discovery_and_prune(&self) -> Result<()> {
        let now = Utc::now().date_naive();
        let min_date = (now - ChronoDuration::days(self.config.retention_days))
            .format("%Y-%m-%d")
            .to_string();
        let max_date = (now + ChronoDuration::days(self.config.upcoming_days))
            .format("%Y-%m-%d")
            .to_string();

        info!(
            "Starting Bollywood/Hindi discovery sync for window [{} .. {}]",
            min_date, max_date
        );

        // 1. Prune movies and series older than 3 months (retention_days)
        let pruned = self.db.prune_older_than(&min_date)?;
        if pruned > 0 {
            info!("Pruned {} expired titles older than {}", pruned, min_date);
        }
        self.db.set_meta("pruned_last_run", &pruned.to_string())?;

        // 2. Discover Movies and Web Series from TMDB
        let discovered = self.tmdb.discover_all(&min_date, &max_date).await?;
        info!(
            "Upserting {} discovered movies & web series into SQLite (single transaction)",
            discovered.len()
        );
        self.db.upsert_batch(&discovered)?;
        drop(discovered);

        // Prune again in case any updated date moved before min_date
        let extra_pruned = self.db.prune_older_than(&min_date)?;
        let total_pruned = pruned + extra_pruned;
        self.db
            .set_meta("pruned_last_run", &total_pruned.to_string())?;
        self.db
            .set_meta("last_discovery_sync", &Utc::now().to_rfc3339())?;

        // 3. Immediately run Homelab status sync so newly discovered items get their Radarr/Sonarr/Seerr badges
        self.run_status_sync().await?;
        self.db.shrink_memory();

        info!("Discovery & status sync completed successfully");
        Ok(())
    }

    pub async fn run_status_sync(&self) -> Result<()> {
        let updated = self.homelab.sync_all_statuses(&self.db).await?;
        self.db
            .set_meta("last_status_sync", &Utc::now().to_rfc3339())?;
        if updated > 0 {
            info!("Updated homelab status for {} items", updated);
        }
        Ok(())
    }
}
