use axum::{
    Router, middleware,
    response::IntoResponse,
    routing::{get, post, put},
};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer,
};

use crate::state::AppState;

use super::handlers::*;
use super::middleware::*;

/// Create the main application router with all routes and middleware
pub fn create_router(req_timeout: u64, state: &AppState) -> Router {
    // API routes
    let api_routes = Router::new()
        .route("/health", get(health_check))
        .route("/config", get(get_config))
        .route("/config", put(update_config))
        .route("/config/backups", get(list_config_backups))
        .route("/config/backups/{filename}", post(restore_config_backup))
        .route("/sensors", get(get_sensors))
        .route("/fans", get(get_fans))
        .route("/data/clear", post(clear_data))
        .route("/data/stats", get(get_stats))
        .route("/alarms", get(get_alarms))
        .route("/alarms", put(update_alarms))
        .with_state(state.clone()); // TODO: is the clone correct? The state should be shared with the sensor thread

    // Nest API routes under /api prefix first, then add fallback
    let app = Router::new()
        .nest("/api", api_routes)
        .fallback(serve_static_files);

    // Add middleware stack
    app.layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(middleware::from_fn(request_logging))
            .layer(CorsLayer::permissive())
            .layer(CompressionLayer::new())
            .layer(TimeoutLayer::new(Duration::from_secs(req_timeout)))
            .layer(middleware::from_fn(error_handler)),
    )
}

/// Fallback handler for serving static files
/// In production, this would serve the frontend files
async fn serve_static_files(
    uri: axum::http::Uri,
) -> axum::response::Result<axum::response::Response> {
    use axum::{http::StatusCode, response::Json};
    use serde_json::json;

    // For now, return a placeholder response
    // TODO: Implement actual static file serving
    Ok((
        StatusCode::NOT_FOUND,
        Json(json!({
            "error": "Static file serving not implemented",
            "requested_path": uri.path(),
            "message": "This will serve the frontend files in production"
        })),
    )
        .into_response())
}
