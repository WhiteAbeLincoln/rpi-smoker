# Task 1.1.3: Implement Configuration Loading from JSON

## Task Type

Backend Development

## Priority

High

## Story Points

5

## Summary

Implement a robust configuration system that loads settings from JSON files with proper validation, defaults, and error handling.

## Description

Create a configuration management system that can load smoker settings from JSON files, provide sensible defaults, validate input values, and handle configuration updates at runtime. This will be the foundation for all hardware and operational settings.

## Acceptance Criteria

- [ ] Configuration loaded from `config/default.json` on startup
- [ ] Environment-specific config override support (e.g., `config/development.json`)
- [ ] CLI argument support for config file path override
- [ ] Configuration validation with detailed error messages
- [ ] Runtime configuration updates (partial updates supported)
- [ ] Configuration backup and restore functionality
- [ ] Proper error handling for missing/invalid config files
- [ ] Unit tests for configuration loading and validation

## Technical Requirements

### Configuration Structure

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfiguration {
    pub server: ServerConfig,
    pub hardware: HardwareConfig,
    pub temp_sensors: HashMap<String, TempSensorConfig>,
    pub fans: HashMap<String, FanConfig>,
    pub alarms: HashMap<String, AlarmConfig>,
    pub data_retention: DataRetentionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
    pub request_timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareConfig {
    pub i2c_bus: u8,
    pub ads1115_address: String,
    pub mock_mode: bool,  // For development/testing
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TempSensorConfig {
    pub ads1115_channel: u8,
    pub gain: f64,
    pub poll_interval_ms: u64,
    pub conversion_formula: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanConfig {
    pub pwm_pin: u8,
    pub frequency_hz: u32,
    pub duty_cycle_formulas: Vec<ConditionalFormula>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalFormula {
    pub when: Option<String>,  // Optional condition
    pub value: String,         // Formula to evaluate
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlarmConfig {
    pub condition: String,
    pub message: String,
    pub actions: Vec<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetentionConfig {
    pub max_age_hours: u64,
    pub max_memory_mb: u64,
    pub cleanup_threshold_percent: f64,
    pub cleanup_percentage: f64,
}
```

### Default Configuration File

Create `config/default.json` with complete example configuration matching the implementation plan.

### Configuration Loading Strategy

1. Load default configuration from `config/default.json`
2. Check for environment-specific override (e.g., `config/development.json`)
3. Apply CLI argument overrides
4. Validate final configuration
5. Return validated config or detailed error

## Implementation Steps

1. Create `src/config/` module:
   - `mod.rs` - Module exports
   - `models.rs` - Configuration structs
   - `loader.rs` - Loading logic
   - `validation.rs` - Validation rules
2. Implement configuration structs with serde derives
3. Create default configuration JSON file
4. Implement configuration loader with file watching
5. Add validation rules for all configuration fields
6. Implement CLI argument parsing for config overrides
7. Add configuration update API endpoint handlers
8. Create configuration backup/restore functionality
9. Write comprehensive unit tests

## Validation Rules

- **Ports**: 1024-65535 range
- **I2C addresses**: Valid hex format (0x00-0xFF)
- **PWM pins**: Valid GPIO pin numbers for Raspberry Pi
- **Poll intervals**: 100ms minimum, 60000ms maximum
- **Formulas**: Basic syntax validation (defer full validation to formula engine)
- **Percentage values**: 0.0-1.0 range
- **Memory limits**: Reasonable bounds (10MB-4GB)

## Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Configuration file not found: {path}")]
    FileNotFound { path: String },

    #[error("Invalid JSON syntax: {source}")]
    InvalidJson { source: serde_json::Error },

    #[error("Validation failed: {field} - {message}")]
    ValidationFailed { field: String, message: String },

    #[error("IO error: {source}")]
    IoError { source: std::io::Error },
}
```

## CLI Integration

```rust
#[derive(Parser)]
#[command(name = "rpi-smoker")]
#[command(about = "Raspberry Pi Smoker Controller")]
pub struct Cli {
    #[arg(short, long, default_value = "config/default.json")]
    pub config: PathBuf,

    #[arg(long)]
    pub port: Option<u16>,

    #[arg(long)]
    pub mock_mode: bool,
}
```

## Definition of Done

- Configuration loads successfully from JSON file
- All validation rules enforce reasonable limits
- CLI arguments properly override file-based config
- Configuration updates work through API
- Comprehensive error messages for common issues
- Unit tests achieve >90% code coverage
- Documentation includes configuration reference

## Dependencies

- Task 1.1.1: Create Cargo workspace with backend crate

## Blocked By

- Task 1.1.1 must be completed first

## Related Tasks

- Task 1.1.2: Set up Axum web server with basic routing
- Task 1.1.4: Create basic data models and validation
- Task 2.1.1: Implement REST API endpoint handlers

## Testing Strategy

- Unit tests for each validation rule
- Integration tests for complete config loading
- Test with malformed JSON files
- Test with missing required fields
- Test CLI argument precedence
- Test configuration updates and persistence

## Notes

- Use `serde` with custom validation functions
- Consider using `config` crate for advanced features
- Implement atomic configuration updates (backup current before applying new)
- Log configuration changes for troubleshooting
- Support partial configuration updates via PATCH-style operations
