# Task 1.1.4: Create Basic Data Models and Validation

## Task Type

Backend Development

## Priority

High

## Story Points

4

## Summary

Define core data models for sensor readings, fan states, alarms, and system statistics with comprehensive validation and serialization support.

## Description

Create the fundamental data structures that will be used throughout the application for representing sensor data, fan states, alarm events, and system statistics. Include proper validation, serialization, and helper methods for data manipulation.

## Acceptance Criteria

- [ ] Core data models defined with proper derives (Serialize, Deserialize, Debug, Clone)
- [ ] Validation functions implemented for all data types
- [ ] Timestamp handling consistent across all models
- [ ] Memory-efficient data structures chosen
- [ ] Unit tests for all validation logic
- [ ] Documentation with examples for each model
- [ ] Error types defined for validation failures
- [ ] Integration with configuration models

## Technical Requirements

### Core Data Models

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SensorReading {
    pub sensor_id: String,
    pub timestamp: DateTime<Utc>,
    pub voltage: f64,
    pub temperature_celsius: f64,
    pub raw_adc_value: Option<u16>,  // For debugging
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FanState {
    pub fan_id: String,
    pub duty_cycle: f64,        // 0.0 to 1.0
    pub pwm_frequency_hz: u32,
    pub timestamp: DateTime<Utc>,
    pub is_manual_override: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlarmEvent {
    pub alarm_id: String,
    pub triggered: bool,
    pub condition: String,      // The formula that triggered
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub acknowledged: bool,
    pub trigger_count: u32,     // How many times this alarm has fired
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataStats {
    pub total_readings: usize,
    pub memory_usage_bytes: usize,
    pub memory_limit_bytes: usize,
    pub oldest_reading: Option<DateTime<Utc>>,
    pub newest_reading: Option<DateTime<Utc>>,
    pub session_duration_seconds: u64,
    pub cleanup_events: u64,
    pub sensors_by_id: HashMap<String, SensorStats>,
    pub fans_by_id: HashMap<String, FanStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorStats {
    pub reading_count: usize,
    pub last_reading: Option<SensorReading>,
    pub avg_temperature: f64,
    pub min_temperature: f64,
    pub max_temperature: f64,
    pub error_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanStats {
    pub state_changes: usize,
    pub current_state: Option<FanState>,
    pub avg_duty_cycle: f64,
    pub total_runtime_seconds: u64,
}
```

### API Response Models

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorsResponse {
    pub sensors: HashMap<String, SensorReading>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FansResponse {
    pub fans: HashMap<String, FanState>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlarmsResponse {
    pub alarms: HashMap<String, AlarmEvent>,
    pub active_count: usize,
    pub timestamp: DateTime<Utc>,
}
```

### Validation Functions

```rust
impl SensorReading {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.sensor_id.is_empty() {
            return Err(ValidationError::EmptyField("sensor_id".to_string()));
        }

        if !(-100.0..=500.0).contains(&self.temperature_celsius) {
            return Err(ValidationError::OutOfRange {
                field: "temperature_celsius".to_string(),
                value: self.temperature_celsius,
                min: -100.0,
                max: 500.0,
            });
        }

        if !(0.0..=5.0).contains(&self.voltage) {
            return Err(ValidationError::OutOfRange {
                field: "voltage".to_string(),
                value: self.voltage,
                min: 0.0,
                max: 5.0,
            });
        }

        Ok(())
    }
}

impl FanState {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.fan_id.is_empty() {
            return Err(ValidationError::EmptyField("fan_id".to_string()));
        }

        if !(0.0..=1.0).contains(&self.duty_cycle) {
            return Err(ValidationError::OutOfRange {
                field: "duty_cycle".to_string(),
                value: self.duty_cycle,
                min: 0.0,
                max: 1.0,
            });
        }

        if !(1..=100000).contains(&self.pwm_frequency_hz) {
            return Err(ValidationError::OutOfRange {
                field: "pwm_frequency_hz".to_string(),
                value: self.pwm_frequency_hz as f64,
                min: 1.0,
                max: 100000.0,
            });
        }

        Ok(())
    }
}
```

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Field '{0}' cannot be empty")]
    EmptyField(String),

    #[error("Field '{field}' value {value} is out of range [{min}, {max}]")]
    OutOfRange {
        field: String,
        value: f64,
        min: f64,
        max: f64,
    },

    #[error("Invalid format for field '{field}': {message}")]
    InvalidFormat { field: String, message: String },

    #[error("Required field '{0}' is missing")]
    MissingField(String),
}
```

## Implementation Steps

1. Create `src/models/` module:
   - `mod.rs` - Module exports
   - `sensor.rs` - Sensor-related models
   - `fan.rs` - Fan-related models
   - `alarm.rs` - Alarm-related models
   - `stats.rs` - Statistics models
   - `api.rs` - API response models
   - `validation.rs` - Validation traits and errors
2. Implement all core data structures
3. Add validation methods to each model
4. Create helper constructors and utility methods
5. Implement conversion between internal and API models
6. Add comprehensive unit tests
7. Create example usage documentation
8. Benchmark memory usage of data structures

## Validation Rules

- **Sensor IDs**: Non-empty, alphanumeric + underscore only
- **Fan IDs**: Non-empty, alphanumeric + underscore only
- **Temperatures**: -100°C to 500°C range
- **Voltages**: 0V to 5V range (ADS1115 range)
- **Duty cycles**: 0.0 to 1.0 (0% to 100%)
- **PWM frequencies**: 1Hz to 100kHz
- **Timestamps**: Not in the future
- **Alarm messages**: Non-empty, max 500 characters

## Memory Considerations

- Use appropriate integer sizes (u16 for ADC values, u32 for frequencies)
- Consider using `Box<str>` instead of `String` for IDs if memory is critical
- Implement `Clone` efficiently (consider `Arc` for shared data)
- Design for efficient serialization/deserialization

## Definition of Done

- All models compile without warnings
- Validation functions catch invalid data
- Unit tests achieve >95% code coverage
- Serialization/deserialization works correctly
- Documentation includes usage examples
- Memory usage benchmarks completed
- Integration tests with configuration models pass

## Dependencies

- Task 1.1.1: Create Cargo workspace with backend crate

## Blocked By

- Task 1.1.1 must be completed first

## Related Tasks

- Task 1.1.3: Implement configuration loading from JSON
- Task 1.3.1: In-memory data store for temperature readings
- Task 2.1.1: Implement REST API endpoint handlers

## Testing Strategy

- Unit tests for each validation function
- Property-based testing for numeric ranges
- Serialization round-trip tests
- Memory usage tests with large datasets
- Integration tests with JSON parsing
- Test error message clarity and helpfulness

## Notes

- Use `chrono::DateTime<Utc>` consistently for all timestamps
- Consider using `uuid` crate for unique identifiers if needed
- Implement `Display` trait for user-friendly error messages
- Use `serde` field attributes for JSON field name mapping
- Consider adding `#[serde(default)]` for optional fields
- Implement proper equality comparison for floating-point fields
