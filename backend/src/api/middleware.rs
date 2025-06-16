use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Json, Response},
};
use chrono::Utc;
use serde_json::json;
use tracing::{error, info};

/// Request logging middleware
pub async fn request_logging(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = std::time::Instant::now();

    info!("Started {} {}", method, uri);

    let response = next.run(request).await;
    let duration = start.elapsed();

    info!(
        "Completed {} {} - {} in {:?}",
        method,
        uri,
        response.status(),
        duration
    );

    response
}

/// Error handling middleware
pub async fn error_handler(request: Request, next: Next) -> Response {
    let response = next.run(request).await;

    // If the response has an error status, we could add additional logging here
    if response.status().is_server_error() {
        error!("Server error occurred: {}", response.status());
    }

    response
}

/// Global error handler for application errors
pub async fn handle_error(err: Box<dyn std::error::Error + Send + Sync>) -> impl IntoResponse {
    error!("Unhandled error: {}", err);

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "error": "Internal server error",
            "timestamp": Utc::now()
        })),
    )
}
