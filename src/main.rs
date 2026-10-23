use anyhow::Result;
use bollyseerr::agent::AgentState;
use bollyseerr::api::build_router;
use bollyseerr::config::Config;
use bollyseerr::db::Database;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config = Config::from_env();
    info!(
        "Starting BollySeerr on port {} (retention={}d, upcoming={}d)",
        config.port, config.retention_days, config.upcoming_days
    );

    let db = Database::open(&config.database_path)?;
    let state = AgentState::new(config.clone(), db);

    state.spawn_background_loops();

    let router = build_router(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = TcpListener::bind(addr).await?;
    info!("BollySeerr HTTP server listening on http://{addr}");

    axum::serve(listener, router).await?;
    Ok(())
}
