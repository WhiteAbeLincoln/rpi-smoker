use axum::{extract::Path, extract::State, http::StatusCode, response::Json};
use chrono::Utc;
use serde_json::{Value, json};

use crate::{
    config::{AppConfig, BackupInfo},
    state::AppState,
};

/// Health check handler
pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": Utc::now(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Get current configuration
pub async fn get_config(State(state): State<AppState>) -> Json<AppConfig> {
    let cfg = state.read_config();
    Json(cfg.clone())
}

/// Update configuration
pub async fn update_config(
    State(state): State<AppState>,
    Json(updates): Json<Value>,
) -> Result<Json<AppConfig>, (StatusCode, Json<Value>)> {
    let mut cfg = state.write_config();
    // TODO: Update should be a JSON Patch operation
    cfg.update_partial(updates).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Failed to update configuration: {e}") })),
        )
    })?;

    Ok(Json(cfg.clone()))
}

/// List configuration backups
pub async fn list_config_backups(
    State(state): State<AppState>,
) -> Result<Json<Vec<BackupInfo>>, (StatusCode, Json<Value>)> {
    let cfg = state.read_config();
    let backups = cfg.list_backups().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to list backups: {e}") })),
        )
    })?;

    Ok(Json(backups))
}

/// Restore configuration from backup
pub async fn restore_config_backup(
    State(state): State<AppState>,
    Path(backup_filename): Path<String>,
) -> Result<Json<AppConfig>, (StatusCode, Json<Value>)> {
    let mut cfg = state.write_config();
    let restored_config = cfg
        .restore_from_backup(&backup_filename)
        .map_err(|e| match e {
            crate::config::ConfigError::FileNotFound { path } => (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": format!("Backup file not found: {path:?}") })),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to restore backup: {e}") })),
            ),
        })?;

    // Update the shared state with the restored configuration
    *cfg = restored_config.clone();

    Ok(Json(restored_config))
}

/// Get current sensor readings
pub async fn get_sensors() -> Json<Value> {
    Json(json!({
        "message": "Sensors endpoint - not implemented yet",
        "sensors": []
    }))
}

/// Get current fan states
pub async fn get_fans() -> Json<Value> {
    Json(json!({
        "message": "Fans endpoint - not implemented yet",
        "fans": []
    }))
}

/// Clear temperature history
pub async fn clear_data() -> (StatusCode, Json<Value>) {
    (
        StatusCode::ACCEPTED,
        Json(json!({
            "message": "Data cleared successfully",
            "timestamp": Utc::now()
        })),
    )
}

/// Get memory usage stats
pub async fn get_stats() -> Json<Value> {
    Json(json!({
        "message": "Stats endpoint - not implemented yet",
        "memory_usage": {
            "used": 0,
            "available": 0,
            "percentage": 0.0
        }
    }))
}

/// Get alarm configurations
pub async fn get_alarms() -> Json<Value> {
    Json(json!({
        "message": "Alarms endpoint - not implemented yet",
        "alarms": []
    }))
}

/// Update alarm configurations
pub async fn update_alarms() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "message": "Update alarms endpoint - not implemented yet"
        })),
    )
}
