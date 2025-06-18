use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::validation::{
    Validate, ValidationError, validate_id_format, validate_range, validate_timestamp_not_future,
};

/// Represents a temperature sensor reading from the ADS1115 ADC
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SensorReading {
    /// Unique identifier for the sensor (e.g., "temp_probe_1")
    pub sensor_id: String,
    /// When this reading was taken
    pub timestamp: DateTime<Utc>,
    /// Voltage reading from the ADC (0.0 to 5.0V)
    pub voltage: f64,
    /// Calculated temperature in Celsius
    pub temperature_celsius: f64,
    /// Raw ADC value for debugging purposes (0-65535 for 16-bit ADC)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_adc_value: Option<u16>,
}

impl SensorReading {
    /// Create a new sensor reading with validation
    pub fn new(
        sensor_id: String,
        voltage: f64,
        temperature_celsius: f64,
        raw_adc_value: Option<u16>,
    ) -> Result<Self, ValidationError> {
        let reading = Self {
            sensor_id,
            timestamp: Utc::now(),
            voltage,
            temperature_celsius,
            raw_adc_value,
        };
        reading.validate()?;
        Ok(reading)
    }

    /// Create a sensor reading with a specific timestamp
    pub fn with_timestamp(
        sensor_id: String,
        timestamp: DateTime<Utc>,
        voltage: f64,
        temperature_celsius: f64,
        raw_adc_value: Option<u16>,
    ) -> Result<Self, ValidationError> {
        let reading = Self {
            sensor_id,
            timestamp,
            voltage,
            temperature_celsius,
            raw_adc_value,
        };
        reading.validate()?;
        Ok(reading)
    }

    /// Check if this reading indicates an overheating condition
    pub fn is_overheating(&self, threshold: f64) -> bool {
        self.temperature_celsius > threshold
    }

    /// Check if this reading indicates an underheating condition
    pub fn is_underheating(&self, threshold: f64) -> bool {
        self.temperature_celsius < threshold
    }

    /// Get the age of this reading in seconds
    pub fn age_seconds(&self) -> i64 {
        (Utc::now() - self.timestamp).num_seconds()
    }
}

impl Validate for SensorReading {
    fn validate(&self) -> Result<(), ValidationError> {
        // Validate sensor ID format
        validate_id_format("sensor_id", &self.sensor_id)?;

        // Validate timestamp is not in the future
        validate_timestamp_not_future("timestamp", &self.timestamp)?;

        // Validate voltage range (ADS1115 with 5V reference)
        validate_range("voltage", self.voltage, 0.0, 5.0)?;

        // Validate temperature range (reasonable for food smoking)
        validate_range(
            "temperature_celsius",
            self.temperature_celsius,
            -100.0,
            500.0,
        )?;

        Ok(())
    }
}

impl std::fmt::Display for SensorReading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sensor {} at {}: {:.1}°C ({:.3}V)",
            self.sensor_id,
            self.timestamp.format("%H:%M:%S"),
            self.temperature_celsius,
            self.voltage
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_sensor_reading_new() {
        let reading = SensorReading::new("test_sensor".to_string(), 3.3, 25.0, Some(2048));
        assert!(reading.is_ok());

        let reading = reading.unwrap();
        assert_eq!(reading.sensor_id, "test_sensor");
        assert_eq!(reading.voltage, 3.3);
        assert_eq!(reading.temperature_celsius, 25.0);
        assert_eq!(reading.raw_adc_value, Some(2048));
    }

    #[test]
    fn test_sensor_reading_validation() {
        // Valid reading
        assert!(SensorReading::new("temp_1".to_string(), 3.3, 25.0, None).is_ok());

        // Invalid sensor ID
        assert!(SensorReading::new("".to_string(), 3.3, 25.0, None).is_err());
        assert!(SensorReading::new("invalid-id".to_string(), 3.3, 25.0, None).is_err());

        // Invalid voltage
        assert!(SensorReading::new("temp_1".to_string(), -1.0, 25.0, None).is_err());
        assert!(SensorReading::new("temp_1".to_string(), 6.0, 25.0, None).is_err());

        // Invalid temperature
        assert!(SensorReading::new("temp_1".to_string(), 3.3, -150.0, None).is_err());
        assert!(SensorReading::new("temp_1".to_string(), 3.3, 600.0, None).is_err());
    }

    #[test]
    fn test_sensor_reading_with_future_timestamp() {
        let future = Utc::now() + Duration::hours(1);
        let reading = SensorReading::with_timestamp("temp_1".to_string(), future, 3.3, 25.0, None);
        assert!(reading.is_err());
    }

    #[test]
    fn test_overheating_underheating() {
        let reading = SensorReading::new("temp_1".to_string(), 3.3, 250.0, None).unwrap();
        assert!(reading.is_overheating(200.0));
        assert!(!reading.is_underheating(200.0));

        let reading = SensorReading::new("temp_1".to_string(), 3.3, 150.0, None).unwrap();
        assert!(!reading.is_overheating(200.0));
        assert!(reading.is_underheating(200.0));
    }

    #[test]
    fn test_age_seconds() {
        let past = Utc::now() - Duration::minutes(5);
        let reading =
            SensorReading::with_timestamp("temp_1".to_string(), past, 3.3, 25.0, None).unwrap();

        let age = reading.age_seconds();
        assert!((295..=305).contains(&age)); // ~300 seconds ± 5 for test execution time
    }

    #[test]
    fn test_display() {
        let reading = SensorReading::new("temp_probe_1".to_string(), 3.3, 25.5, None).unwrap();
        let display = format!("{reading}");
        assert!(display.contains("temp_probe_1"));
        assert!(display.contains("25.5°C"));
        assert!(display.contains("3.300V"));
    }

    #[test]
    fn test_serialization() {
        let reading = SensorReading::new("temp_1".to_string(), 3.3, 25.0, Some(2048)).unwrap();

        // Test serialization
        let json = serde_json::to_string(&reading).unwrap();
        assert!(json.contains("temp_1"));
        assert!(json.contains("3.3"));
        assert!(json.contains("25.0"));
        assert!(json.contains("2048"));

        // Test deserialization
        let deserialized: SensorReading = serde_json::from_str(&json).unwrap();
        assert_eq!(reading, deserialized);
    }
}
