use bollyseerr::agent::AgentState;
use bollyseerr::config::Config;
use bollyseerr::db::Database;
use bollyseerr::models::{HomelabStatus, MediaItem, MediaType};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn sample_item(tmdb_id: i64, media_type: MediaType, title: &str, release_date: &str) -> MediaItem {
    MediaItem {
        tmdb_id,
        media_type,
        title: title.to_string(),
        original_title: title.to_string(),
        original_language: "hi".to_string(),
        is_hindi_dub: false,
        overview: "Test Bollywood synopsis".to_string(),
        poster_path: Some("/poster.jpg".to_string()),
        backdrop_path: Some("/backdrop.jpg".to_string()),
        release_date: release_date.to_string(),
        vote_average: 7.8,
        vote_count: 120,
        popularity: 45.0,
        genres: vec!["Action".to_string(), "Thriller".to_string()],
        ott_providers: vec!["Netflix".to_string()],
        release_type: "Theatrical".to_string(),
        trailer_key: Some("abc123xyz".to_string()),
        season_count: None,
        latest_season_number: None,
        homelab_status: HomelabStatus::Unrequested,
        radarr_id: None,
        sonarr_id: None,
        seerr_request_id: None,
        updated_at: "2026-10-23T00:00:00Z".to_string(),
    }
}

#[test]
fn test_90_day_pruning_and_upsert() {
    let db = Database::open(":memory:").expect("open in-memory db");

    // 1. Movie older than 90 days (e.g., 2026-06-01 when cutoff is 2026-07-25)
    let old_movie = sample_item(101, MediaType::Movie, "Old Hindi Movie", "2026-06-01");
    // 2. Movie released within last 90 days
    let recent_movie = sample_item(102, MediaType::Movie, "Stree 3", "2026-09-15");
    // 3. Upcoming web series
    let upcoming_series = sample_item(201, MediaType::Tv, "Panchayat S4", "2026-11-20");

    db.upsert_item(&old_movie).unwrap();
    db.upsert_item(&recent_movie).unwrap();
    db.upsert_item(&upcoming_series).unwrap();

    assert_eq!(db.get_all_items().unwrap().len(), 3);

    let pruned = db.prune_older_than("2026-07-25").unwrap();
    assert_eq!(pruned, 1);

    let remaining = db.get_all_items().unwrap();
    assert_eq!(remaining.len(), 2);
    assert!(remaining.iter().any(|i| i.tmdb_id == 102));
    assert!(remaining.iter().any(|i| i.tmdb_id == 201));
    assert!(!remaining.iter().any(|i| i.tmdb_id == 101));
}

#[tokio::test]
async fn test_homelab_status_sync_and_seerr_request() {
    let mock_server = MockServer::start().await;

    // Mock Radarr returning movie 102 as downloaded (hasFile = true)
    Mock::given(method("GET"))
        .and(path("/api/v3/movie"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": 55,
                "tmdbId": 102,
                "monitored": true,
                "hasFile": true
            }
        ])))
        .mount(&mock_server)
        .await;

    // Mock Sonarr returning series 201 as monitored (waiting for episodes)
    Mock::given(method("GET"))
        .and(path("/api/v3/series"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": 77,
                "tmdbId": 201,
                "monitored": true,
                "statistics": {
                    "episodeFileCount": 0,
                    "episodeCount": 8
                }
            }
        ])))
        .mount(&mock_server)
        .await;

    // Mock Seerr request list & create request
    Mock::given(method("GET"))
        .and(path("/api/v1/request"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": []
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/api/v1/request"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": 999,
            "status": 2
        })))
        .mount(&mock_server)
        .await;

    let db = Database::open(":memory:").unwrap();
    db.upsert_item(&sample_item(102, MediaType::Movie, "Stree 3", "2026-09-15"))
        .unwrap();
    db.upsert_item(&sample_item(
        201,
        MediaType::Tv,
        "Panchayat S4",
        "2026-11-20",
    ))
    .unwrap();
    db.upsert_item(&sample_item(103, MediaType::Movie, "War 2", "2026-12-01"))
        .unwrap();

    let cfg = Config {
        port: 5056,
        database_path: ":memory:".to_string(),
        tmdb_api_key: "test".to_string(),
        tmdb_base_url: mock_server.uri(),
        seerr_url: mock_server.uri(),
        seerr_api_key: Some("seerr-secret".to_string()),
        seerr_settings_path: "/nonexistent".to_string(),
        radarr_url: mock_server.uri(),
        radarr_api_key: Some("radarr-secret".to_string()),
        sonarr_url: mock_server.uri(),
        sonarr_api_key: Some("sonarr-secret".to_string()),
        retention_days: 90,
        upcoming_days: 180,
        discovery_interval_seconds: 3600,
        status_sync_interval_seconds: 300,
        max_pages_per_query: 1,
    };

    let state = AgentState::new(cfg, db.clone());
    state.run_status_sync().await.unwrap();

    let m102 = db.get_item(102, MediaType::Movie).unwrap().unwrap();
    assert_eq!(m102.homelab_status, HomelabStatus::Available);
    assert_eq!(m102.radarr_id, Some(55));

    let s201 = db.get_item(201, MediaType::Tv).unwrap().unwrap();
    assert_eq!(s201.homelab_status, HomelabStatus::Monitored);
    assert_eq!(s201.sonarr_id, Some(77));

    // Request movie 103 via Seerr
    let req_res = state
        .homelab
        .request_media(&db, 103, MediaType::Movie, None)
        .await
        .unwrap();
    assert!(req_res.success);
    assert_eq!(req_res.via, "seerr");

    let m103 = db.get_item(103, MediaType::Movie).unwrap().unwrap();
    assert_eq!(m103.homelab_status, HomelabStatus::Requested);
    assert_eq!(m103.seerr_request_id, Some(999));
}
