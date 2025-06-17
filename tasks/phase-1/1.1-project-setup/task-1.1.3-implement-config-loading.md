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

- [ ] On startup, onfiguration loaded from the config file path provided
      by the CLI argument, or `config.json` in the current working directory
- [ ] CLI argument support for overriding ServerConfig fields
- [ ] Configuration validation with detailed error messages
- [ ] API for runtime configuration updates (partial updates supported)
- [ ] When config is changed through API, back up previous config in a `config_backup` directory
- [ ] API for configuration restore functionality
- [ ] Proper error handling for missing/invalid config files
- [ ] Unit tests for configuration loading and validation

## Technical Requirements

### Configuration Structure

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
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
    /// the backup directory for configuration
    pub config_backup_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareConfig {
    pub ads1115: ADS1115Config,
    pub mock_mode: bool,  // For development/testing
}

// TODO: convert to FullScaleRange from ads1x1x
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub enum ADSGain {
    /// The measurable range is ±6.144V.
    TwoThirds,
    /// The measurable range is ±4.096V.
    One,
    /// The measurable range is ±2.048V. (default)
    #[default]
    Two,
    /// The measurable range is ±1.024V.
    Four,
    /// The measurable range is ±0.512V.
    Eight,
    /// The measurable range is ±0.256V.
    Sixteen,
}

impl ADSGain {
    fn to_volts(self) -> f64 {
        match self {
            ADSGain::TwoThirds => 6.144,
            ADSGain::One => 4.096,
            ADSGain::Two => 2.048,
            ADSGain::Four => 1.024,
            ADSGain::Eight => 0.512,
            ADSGain::Sixteen => 0.256,
        }
    }
}

use serde_repr::*;
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
#[repr(u8)]
pub enum ADSChannel {
    #[default]
    A0 = 0,
    A1 = 1,
    A2 = 2,
    A3 = 3,
}

// TODO: convert to DataRate16Bit from ads1x1x
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
#[repr(u16)]
pub enum ADSDataRate {
    Rate8SPS = 8,
    Rate16SPS = 16,
    Rate32SPS = 32,
    Rate64SPS = 64,
    #[default]
    Rate128SPS = 128,
    Rate250SPS = 250,
    Rate475SPS = 475,
    Rate860SPS = 860,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ADS1115Config {
    /// the i2c bus that the ADS1115 is connected to
    pub i2c_bus: Option<u8>,
    /// optional address
    pub address: Option<u16>,
    pub gain: ADSGain,
    pub data_rate: ADSDataRate,
    pub poll_interval_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TempSensorConfig {
    pub ads_channel: ADSChannel,
    /// conversion from volts to degrees celsius
    pub conversion_formula: String,
    // not deserialized, this is the name in the hashmap for easy reference
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanConfig {
    pub pwm_pin: u8,
    pub frequency_hz: u32,
    pub duty_cycle_formulas: Vec<ConditionalFormula>,
    // not deserialized, this is the name in the hashmap for easy reference
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalFormula {
    pub when: Option<String>,  // Optional condition
    pub value: String,         // Formula to evaluate
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlarmConfig {
    // not deserialized, this is the name in the hashmap for easy reference
    pub name: String,
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

### Configuration Loading Strategy

1. Load configuration from `config.json` by default, or CLI argument path
2. Validate configuration
3. Return validated config or detailed error

## Implementation Steps

1. Create `src/config/` module:
   - `mod.rs` - Module exports
   - `models.rs` - Configuration structs
   - `loader.rs` - Loading logic
   - `validation.rs` - Validation rules
2. Implement configuration structs with serde derives
3. Implement configuration loader. File watching is
   not necessary, the user should restart the application after manual config changes.
4. Add validation rules for all configuration fields
5. Implement CLI argument parsing for config path and ServerConfig overrides
6. Add configuration update API endpoint handlers
7. Create configuration backup/restore functionality
8. Write comprehensive unit tests

## Validation Rules

- **Ports**: 1024-65535 range
- **I2C addresses**: Valid hex format (0x00-0xFF)
- **PWM pins**: Valid GPIO pin numbers for Raspberry Pi
- **Poll intervals**: 250ms minimum, 60000ms maximum
- **Formulas**: (defer full validation to formula engine)
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
