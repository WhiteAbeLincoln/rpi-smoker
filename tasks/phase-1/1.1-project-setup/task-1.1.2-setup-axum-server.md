# Task 1.1.2: Set up Axum Web Server with Basic Routing

## Task Type

Backend Development

## Priority

High

## Story Points

5

## Summary

Implement a basic Axum web server with initial routing structure and middleware configuration for the RPI Smoker API.

## Description

Create the foundational web server using Axum framework that will serve both the REST API and static frontend files. Set up basic routing structure, error handling, and essential middleware.

## Acceptance Criteria

- [ ] Axum server starts successfully on configurable port (default 3000)
- [ ] Basic route structure implemented with placeholder handlers
- [ ] CORS middleware configured for frontend development
- [ ] JSON response/request handling working
- [ ] Basic error handling and logging implemented
- [ ] Health check endpoint responding correctly
- [ ] Server graceful shutdown implemented

## Technical Requirements

### Routes to Implement

```rust
// API routes (placeholder handlers)
GET  /api/health                    # Health check
GET  /api/config                    # Get current configuration
PUT  /api/config                    # Update configuration
GET  /api/sensors                   # Get current sensor readings
GET  /api/fans                      # Get current fan states
POST /api/data/clear                # Clear temperature history
GET  /api/data/stats                # Get memory usage stats
GET  /api/alarms                    # Get alarm configurations
PUT  /api/alarms                    # Update alarm configurations

// Static file serving
GET  /*path                         # Serve frontend files (fallback to index.html)
```

### Middleware Stack Required

1. Request logging
2. CORS (allow all origins for development)
3. JSON body parsing
4. Response compression (optional)
5. Request timeout
6. Error handling

### Configuration Structure

```rust
#[derive(Clone)]
pub struct AppConfig {
    pub port: u16,
    pub host: String,
    pub cors_origins: Vec<String>,
    pub request_timeout_seconds: u64,
}
```

## Implementation Steps

1. Create `src/api/` module structure:
   - `mod.rs` - Module exports
   - `routes.rs` - Route definitions
   - `handlers.rs` - Request handlers
   - `middleware.rs` - Custom middleware
   - `models.rs` - Request/response models
2. Implement basic request/response types
3. Set up router with all required endpoints
4. Add middleware stack
5. Implement graceful shutdown with signal handling
6. Add basic logging configuration
7. Create health check handler
8. Test all endpoints return appropriate HTTP status codes

## Example Handler Implementation

```rust
// Health check handler
pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "timestamp": Utc::now(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

// Placeholder for configuration endpoint
pub async fn get_config() -> impl IntoResponse {
    Json(json!({
        "message": "Configuration endpoint - not implemented yet"
    }))
}
```

## Definition of Done

- Server starts and serves on specified port
- All defined routes return valid HTTP responses (even if placeholder)
- CORS headers properly set for frontend development
- Request/response logging working
- Graceful shutdown implemented (responds to SIGTERM/SIGINT)
- Basic error responses return proper JSON format
- No compilation warnings

## Dependencies

- Task 1.1.1: Create Cargo workspace with backend crate

## Blocked By

- Task 1.1.1 must be completed first

## Related Tasks

- Task 1.1.3: Implement configuration loading from JSON
- Task 1.1.4: Create basic data models and validation
- Task 2.1.1: Implement REST API endpoint handlers

## Testing Notes

- Test server startup and shutdown
- Verify CORS headers with browser dev tools
- Test each route returns appropriate HTTP status
- Verify JSON response format consistency
- Test with invalid JSON requests

## Notes

- Use `tower-http` for middleware
- Configure reasonable request timeouts (30 seconds)
- Ensure proper error propagation from handlers
- Consider using `tracing` crate for structured logging
- Implement proper content-type handling for JSON
