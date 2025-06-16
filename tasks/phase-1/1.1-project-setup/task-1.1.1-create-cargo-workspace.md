# Task 1.1.1: Create Cargo Workspace with Backend Crate

## Task Type

Setup/Infrastructure

## Priority

High

## Story Points

3

## Summary

Create a Cargo workspace configuration and initialize the backend crate structure for the RPI Smoker project.

## Description

Set up the foundational Cargo workspace that will contain multiple crates (backend, shared types, etc.) and create the initial backend crate with proper structure.

## Acceptance Criteria

- [ ] Root `Cargo.toml` file created with workspace configuration
- [ ] `backend/` directory created with its own `Cargo.toml`
- [ ] Backend crate includes required dependencies for Axum web server
- [ ] Workspace builds successfully with `cargo build`
- [ ] Backend crate can be run with `cargo run -p backend`
- [ ] Proper feature flags configured for cross-platform development (Pi vs dev)

## Technical Requirements

### Root Cargo.toml Structure

```toml
[workspace]
members = ["backend", "shared"]
default-members = ["backend"]

[workspace.dependencies]
# Common dependencies shared across crates
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
```

### Backend Dependencies Required

- `axum` - Web framework
- `tokio` - Async runtime
- `serde` - Serialization
- `serde_json` - JSON handling
- `tower` - Middleware
- `tower-http` - HTTP middleware (CORS, etc.)
- `clap` - CLI argument parsing
- `rppal` - Raspberry Pi GPIO (with conditional compilation)

### Directory Structure to Create

```
backend/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── lib.rs
└── tests/
```

## Implementation Steps

1. Create root `Cargo.toml` with workspace configuration
2. Create `backend/` directory
3. Initialize backend crate with `cargo init backend --name rpi-smoker-backend`
4. Add required dependencies to backend `Cargo.toml`
5. Set up feature flags for hardware dependencies
6. Create basic `main.rs` with placeholder Axum server
7. Test compilation on both development machine and target platform

## Definition of Done

- Workspace compiles without errors
- Basic "Hello World" Axum server starts and responds
- Feature flags properly exclude hardware dependencies on non-Pi platforms
- Code follows Rust best practices and formatting standards

## Dependencies

None

## Blocked By

None

## Related Tasks

- Task 1.1.2: Set up Axum web server with basic routing
- Task 1.2.1: Design trait-based hardware interface

## Notes

- Use feature flags like `rpi-hardware` to conditionally compile GPIO-related code
- Ensure cross-compilation compatibility for ARM targets
- Consider using `cargo-generate` template for consistent structure
