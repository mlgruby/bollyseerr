use crate::agent::AgentState;
use crate::models::{HomelabStatus, RequestPayload, RequestResult, SyncStatus};
use crate::ui::INDEX_HTML;
use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::Utc;
use serde_json::json;
use std::sync::atomic::Ordering;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub fn build_router(state: AgentState) -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/health", get(health_handler))
        .route("/status", get(health_handler))
        .route("/api/status", get(sync_status_handler))
        .route("/api/items", get(list_items_handler))
        .route("/api/request", post(request_item_handler))
        .route("/api/sync", post(trigger_sync_handler))
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn index_handler() -> Html<&'static str> {
    Html(INDEX_HTML)
}

async fn health_handler(State(state): State<AgentState>) -> impl IntoResponse {
    let today = Utc::now().date_naive().format("%Y-%m-%d").to_string();
    let is_syncing = state.is_syncing.load(Ordering::SeqCst);
    match state.db.get_sync_status(&today, is_syncing) {
        Ok(st) => (
            StatusCode::OK,
            Json(json!({
                "status": "ok",
                "service": "bollyseerr",
                "movies": st.total_movies,
                "series": st.total_series,
                "is_syncing": st.is_syncing,
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": e.to_string()
            })),
        ),
    }
}

async fn sync_status_handler(
    State(state): State<AgentState>,
) -> Result<Json<SyncStatus>, (StatusCode, String)> {
    let today = Utc::now().date_naive().format("%Y-%m-%d").to_string();
    let is_syncing = state.is_syncing.load(Ordering::SeqCst);
    state
        .db
        .get_sync_status(&today, is_syncing)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn list_items_handler(
    State(state): State<AgentState>,
) -> Result<Response, (StatusCode, String)> {
    let payload = state
        .db
        .get_all_items_json()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        payload.to_string(),
    )
        .into_response())
}

async fn request_item_handler(
    State(state): State<AgentState>,
    Json(payload): Json<RequestPayload>,
) -> impl IntoResponse {
    match state
        .homelab
        .request_media(
            &state.db,
            payload.tmdb_id,
            payload.media_type,
            payload.seasons,
        )
        .await
    {
        Ok(res) => {
            // Spawn immediate background status sync to reconcile Radarr/Sonarr/Seerr state
            let bg_state = state.clone();
            tokio::spawn(async move {
                let _ = bg_state.run_status_sync().await;
            });
            (StatusCode::OK, Json(res))
        }
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            Json(RequestResult {
                success: false,
                status: HomelabStatus::Unrequested,
                message: format!("{e:#}"),
                via: "none".to_string(),
            }),
        ),
    }
}

async fn trigger_sync_handler(State(state): State<AgentState>) -> impl IntoResponse {
    let bg_state = state.clone();
    tokio::spawn(async move {
        let _ = bg_state.run_discovery_and_prune().await;
    });
    (
        StatusCode::ACCEPTED,
        Json(json!({
            "triggered": true,
            "message": "Background TMDB discovery & homelab status sync started"
        })),
    )
}
