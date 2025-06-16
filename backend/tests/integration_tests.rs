#[tokio::test]
async fn test_basic_functionality() {
    // Basic test to ensure the crate compiles and basic functionality works
    let config = rpi_smoker_backend::config::AppConfig::default();
    assert_eq!(config.server.port, 3000);
    assert_eq!(config.server.host, "0.0.0.0");
}

#[tokio::test]
async fn test_error_handling() {
    use rpi_smoker_backend::AppError;

    let error = AppError::BadRequest("Test error".to_string());
    assert!(error.to_string().contains("Bad request"));
}

#[tokio::test]
async fn test_error_convenience_constructors() {
    use rpi_smoker_backend::AppError;

    // Test convenience constructors
    let bad_request = AppError::bad_request("Invalid input");
    assert!(bad_request.to_string().contains("Bad request: Invalid input"));

    let not_found = AppError::not_found("Resource not found");
    assert!(not_found.to_string().contains("Not found: Resource not found"));

    let validation = AppError::validation("Invalid format");
    assert!(validation.to_string().contains("Validation error: Invalid format"));

    let config = AppError::config("Missing required setting");
    assert!(config.to_string().contains("Configuration error: Missing required setting"));
}

#[tokio::test]
async fn test_error_from_implementations() {
    use rpi_smoker_backend::AppError;
    use std::io;

    // Test automatic conversion from std::io::Error
    let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let app_error: AppError = io_error.into();
    assert!(app_error.to_string().contains("IO error"));

    // Test automatic conversion from serde_json::Error
    let json_result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str("invalid json");
    if let Err(json_error) = json_result {
        let app_error: AppError = json_error.into();
        assert!(app_error.to_string().contains("JSON serialization error"));
    }
}
