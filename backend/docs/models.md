# Data Models Documentation

This document provides examples and usage patterns for the data models implemented in the rpi-smoker backend.

## Overview

The data models provide type-safe representations of sensor readings, fan states, alarm events, and system statistics. All models include comprehensive validation and serialization support.

## Core Models

### SensorReading

Represents a temperature sensor reading from the ADS1115 ADC.

```rust
use rpi_smoker::models::SensorReading;

// Create a new sensor reading
let reading = SensorReading::new(
    "temp_probe_1".to_string(),
    3.3,     // voltage
    225.5,   // temperature in Celsius
    Some(2048), // raw ADC value (optional)
)?;

// Check for overheating
if reading.is_overheating(250.0) {
    println!("Temperature too high: {}", reading);
}

// Get reading age
println!("Reading is {} seconds old", reading.age_seconds());
```

### FanState

Represents the current state of a PWM-controlled fan.

```rust
use rpi_smoker::models::FanState;

// Create a new fan state
let state = FanState::new(
    "intake_fan".to_string(),
    0.75,    // 75% duty cycle
    25000,   // 25kHz PWM frequency
    false,   // not manual override
)?;

// Create convenience states
let off_state = FanState::off("exhaust_fan".to_string())?;
let full_state = FanState::full_speed("intake_fan".to_string(), 25000)?;

// Modify duty cycle
let new_state = state.with_duty_cycle(0.5)?;

// Check if running
if state.is_running() {
    println!("Fan {} is running at {:.1}%",
             state.fan_id,
             state.duty_cycle_percent());
}
```

### AlarmEvent

Represents an alarm that can be triggered by various conditions. Note that the condition and message are stored in the AlarmConfig and can be retrieved using the alarm_id.

```rust
use rpi_smoker::models::AlarmEvent;

// Create a triggered alarm
let mut alarm = AlarmEvent::triggered(
    "high_temp_alarm".to_string(),
)?;

// Check severity
match alarm.severity_level() {
    AlarmSeverity::Critical => println!("CRITICAL ALARM!"),
    AlarmSeverity::High => println!("High priority alarm"),
    _ => println!("Standard alarm"),
}

// Acknowledge the alarm
if alarm.needs_attention() {
    alarm.acknowledge();
}

// Trigger again (increments count)
alarm.trigger();
println!("Alarm triggered {} times", alarm.trigger_count);

// Note: To get the condition and message, look up the alarm_id in AlarmConfig
// let alarm_config = config.alarms.get(&alarm.alarm_id).unwrap();
// println!("Condition: {}", alarm_config.condition);
// println!("Message: {}", alarm_config.message);
```

### Statistics Models

Track system performance and data usage.

```rust
use rpi_smoker::models::{DataStats, SensorStats, FanStats};

// Create system statistics
let mut stats = DataStats::new(1024 * 1024); // 1MB limit

// Check memory usage
if stats.is_memory_usage_high() {
    println!("Memory usage: {:.1}%", stats.memory_usage_percent());
}

// Update sensor statistics
let mut sensor_stats = SensorStats::new();
sensor_stats.update_with_reading(&reading);
println!("Average temperature: {:.1}°C", sensor_stats.avg_temperature);

// Update fan statistics
let mut fan_stats = FanStats::new();
fan_stats.update_with_state(&state);
println!("Fan runtime: {} seconds", fan_stats.total_runtime_seconds);
```

## API Response Models

### ApiResponse<T>

Generic wrapper for API responses.

```rust
use rpi_smoker::models::ApiResponse;

// Success response
let response = ApiResponse::success(reading);
assert!(response.success);
assert!(response.data.is_some());

// Error response
let error_response = ApiResponse::<SensorReading>::error(
    "Sensor not found".to_string()
);
assert!(!error_response.success);
assert!(error_response.error.is_some());
```

### Specialized Response Types

```rust
use rpi_smoker::models::{SensorsResponse, FansResponse, AlarmsResponse};
use std::collections::HashMap;

// Sensors response
let mut sensors = HashMap::new();
sensors.insert("temp_1".to_string(), reading);
let sensors_response = SensorsResponse::new(sensors);

println!("Average temperature: {:.1}°C",
         sensors_response.average_temperature().unwrap_or(0.0));

// Fans response
let mut fans = HashMap::new();
fans.insert("fan_1".to_string(), state);
let fans_response = FansResponse::new(fans);

println!("Running fans: {}/{}",
         fans_response.running_fan_count(),
         fans_response.fan_count());

// Alarms response
let mut alarms = HashMap::new();
alarms.insert("alarm_1".to_string(), alarm);
let alarms_response = AlarmsResponse::new(alarms);

if alarms_response.has_critical_alarms() {
    println!("CRITICAL ALARMS ACTIVE!");
}
```

## Validation

All models implement the `Validate` trait and perform validation on creation.

```rust
use rpi_smoker::models::{SensorReading, ValidationError};

// This will fail validation
let invalid_reading = SensorReading::new(
    "".to_string(),        // empty sensor ID
    10.0,                  // voltage out of range
    600.0,                 // temperature out of range
    None,
);

match invalid_reading {
    Ok(_) => println!("Reading is valid"),
    Err(ValidationError::EmptyField(field)) => {
        println!("Field '{}' cannot be empty", field);
    },
    Err(ValidationError::OutOfRange { field, value, min, max }) => {
        println!("Field '{}' value {} is out of range [{}, {}]",
                 field, value, min, max);
    },
    Err(e) => println!("Validation error: {}", e),
}
```

## Serialization

All models support JSON serialization/deserialization via serde.

```rust
use serde_json;

// Serialize to JSON
let json = serde_json::to_string(&reading)?;
println!("JSON: {}", json);

// Deserialize from JSON
let deserialized: SensorReading = serde_json::from_str(&json)?;
assert_eq!(reading, deserialized);

// Pretty print JSON
let pretty_json = serde_json::to_string_pretty(&fans_response)?;
println!("Pretty JSON:\n{}", pretty_json);
```

## Error Handling

The models use the `ValidationError` type for validation failures.

```rust
use rpi_smoker::models::ValidationError;

fn handle_validation_error(error: ValidationError) {
    match error {
        ValidationError::EmptyField(field) => {
            eprintln!("Required field '{}' is missing", field);
        },
        ValidationError::OutOfRange { field, value, min, max } => {
            eprintln!("Value {} for '{}' must be between {} and {}",
                     value, field, min, max);
        },
        ValidationError::InvalidFormat { field, message } => {
            eprintln!("Invalid format for '{}': {}", field, message);
        },
        ValidationError::MissingField(field) => {
            eprintln!("Missing required field: {}", field);
        },
    }
}
```

## Memory Considerations

The models are designed for efficient memory usage:

- Use appropriate integer sizes (u16 for ADC values, u32 for frequencies)
- Implement `Clone` efficiently
- Support efficient serialization/deserialization
- Include optional fields where appropriate to reduce memory footprint

## Testing

All models include comprehensive unit tests. Run tests with:

```bash
cargo test models
```

For property-based testing and benchmarking:

```bash
cargo test --release models::tests
```
