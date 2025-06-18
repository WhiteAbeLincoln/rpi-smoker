use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::validation::{
    Validate, ValidationError, validate_id_format, validate_range, validate_timestamp_not_future,
};

/// Represents the current state of a PWM-controlled fan
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FanState {
    /// Unique identifier for the fan (e.g., "intake_fan", "exhaust_fan")
    pub fan_id: String,
    /// PWM duty cycle from 0.0 (off) to 1.0 (full speed)
    pub duty_cycle: f64,
    /// PWM frequency in Hz
    pub pwm_frequency_hz: u32,
    /// When this state was set
    pub timestamp: DateTime<Utc>,
    /// Whether this is a manual override or automatic control
    pub is_manual_override: bool,
}

impl FanState {
    /// Create a new fan state with validation
    pub fn new(
        fan_id: String,
        duty_cycle: f64,
        pwm_frequency_hz: u32,
        is_manual_override: bool,
    ) -> Result<Self, ValidationError> {
        let state = Self {
            fan_id,
            duty_cycle,
            pwm_frequency_hz,
            timestamp: Utc::now(),
            is_manual_override,
        };
        state.validate()?;
        Ok(state)
    }

    /// Create a fan state with a specific timestamp
    pub fn with_timestamp(
        fan_id: String,
        duty_cycle: f64,
        pwm_frequency_hz: u32,
        timestamp: DateTime<Utc>,
        is_manual_override: bool,
    ) -> Result<Self, ValidationError> {
        let state = Self {
            fan_id,
            duty_cycle,
            pwm_frequency_hz,
            timestamp,
            is_manual_override,
        };
        state.validate()?;
        Ok(state)
    }

    /// Create a fan state for turning the fan off
    pub fn off(fan_id: String) -> Result<Self, ValidationError> {
        Self::new(fan_id, 0.0, 1000, false) // Default 1kHz frequency when off
    }

    /// Create a fan state for running the fan at full speed
    pub fn full_speed(fan_id: String, pwm_frequency_hz: u32) -> Result<Self, ValidationError> {
        Self::new(fan_id, 1.0, pwm_frequency_hz, false)
    }

    /// Check if the fan is currently running (duty cycle > 0)
    pub fn is_running(&self) -> bool {
        self.duty_cycle > 0.0
    }

    /// Check if the fan is at full speed
    pub fn is_full_speed(&self) -> bool {
        self.duty_cycle >= 1.0
    }

    /// Get the duty cycle as a percentage (0-100)
    pub fn duty_cycle_percent(&self) -> f64 {
        self.duty_cycle * 100.0
    }

    /// Get the age of this state in seconds
    pub fn age_seconds(&self) -> i64 {
        (Utc::now() - self.timestamp).num_seconds()
    }

    /// Create a new state with the same parameters but updated duty cycle
    pub fn with_duty_cycle(&self, duty_cycle: f64) -> Result<Self, ValidationError> {
        Self::new(
            self.fan_id.clone(),
            duty_cycle,
            self.pwm_frequency_hz,
            self.is_manual_override,
        )
    }

    /// Create a new state marking it as manual override
    pub fn as_manual_override(&self) -> Result<Self, ValidationError> {
        Self::new(
            self.fan_id.clone(),
            self.duty_cycle,
            self.pwm_frequency_hz,
            true,
        )
    }

    /// Create a new state marking it as automatic control
    pub fn as_automatic(&self) -> Result<Self, ValidationError> {
        Self::new(
            self.fan_id.clone(),
            self.duty_cycle,
            self.pwm_frequency_hz,
            false,
        )
    }
}

impl Validate for FanState {
    fn validate(&self) -> Result<(), ValidationError> {
        // Validate fan ID format
        validate_id_format("fan_id", &self.fan_id)?;

        // Validate timestamp is not in the future
        validate_timestamp_not_future("timestamp", &self.timestamp)?;

        // Validate duty cycle range (0.0 to 1.0)
        validate_range("duty_cycle", self.duty_cycle, 0.0, 1.0)?;

        // Validate PWM frequency range (1Hz to 100kHz)
        validate_range(
            "pwm_frequency_hz",
            self.pwm_frequency_hz as f64,
            1.0,
            100000.0,
        )?;

        Ok(())
    }
}

impl std::fmt::Display for FanState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = if self.is_manual_override {
            "MANUAL"
        } else {
            "AUTO"
        };

        write!(
            f,
            "Fan {} at {}: {:.1}% ({:.0}Hz) [{}]",
            self.fan_id,
            self.timestamp.format("%H:%M:%S"),
            self.duty_cycle_percent(),
            self.pwm_frequency_hz,
            mode
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_fan_state_new() {
        let state = FanState::new("intake_fan".to_string(), 0.75, 25000, false);
        assert!(state.is_ok());

        let state = state.unwrap();
        assert_eq!(state.fan_id, "intake_fan");
        assert_eq!(state.duty_cycle, 0.75);
        assert_eq!(state.pwm_frequency_hz, 25000);
        assert!(!state.is_manual_override);
    }

    #[test]
    fn test_fan_state_validation() {
        // Valid state
        assert!(FanState::new("fan_1".to_string(), 0.5, 1000, false).is_ok());

        // Invalid fan ID
        assert!(FanState::new("".to_string(), 0.5, 1000, false).is_err());
        assert!(FanState::new("invalid-id".to_string(), 0.5, 1000, false).is_err());

        // Invalid duty cycle
        assert!(FanState::new("fan_1".to_string(), -0.1, 1000, false).is_err());
        assert!(FanState::new("fan_1".to_string(), 1.1, 1000, false).is_err());

        // Invalid PWM frequency
        assert!(FanState::new("fan_1".to_string(), 0.5, 0, false).is_err());
        assert!(FanState::new("fan_1".to_string(), 0.5, 200000, false).is_err());
    }

    #[test]
    fn test_fan_state_with_future_timestamp() {
        let future = Utc::now() + Duration::hours(1);
        let state = FanState::with_timestamp("fan_1".to_string(), 0.5, 1000, future, false);
        assert!(state.is_err());
    }

    #[test]
    fn test_fan_state_convenience_constructors() {
        // Test off
        let off_state = FanState::off("fan_1".to_string()).unwrap();
        assert_eq!(off_state.duty_cycle, 0.0);
        assert!(!off_state.is_running());

        // Test full speed
        let full_state = FanState::full_speed("fan_1".to_string(), 25000).unwrap();
        assert_eq!(full_state.duty_cycle, 1.0);
        assert!(full_state.is_full_speed());
        assert!(full_state.is_running());
    }

    #[test]
    fn test_fan_state_properties() {
        let state = FanState::new("fan_1".to_string(), 0.75, 25000, false).unwrap();

        assert!(state.is_running());
        assert!(!state.is_full_speed());
        assert_eq!(state.duty_cycle_percent(), 75.0);
    }

    #[test]
    fn test_fan_state_modifications() {
        let original = FanState::new("fan_1".to_string(), 0.5, 25000, false).unwrap();

        // Test duty cycle change
        let modified = original.with_duty_cycle(0.8).unwrap();
        assert_eq!(modified.duty_cycle, 0.8);
        assert_eq!(modified.fan_id, original.fan_id);

        // Test manual override
        let manual = original.as_manual_override().unwrap();
        assert!(manual.is_manual_override);

        // Test automatic
        let auto = manual.as_automatic().unwrap();
        assert!(!auto.is_manual_override);
    }

    #[test]
    fn test_age_seconds() {
        let past = Utc::now() - Duration::minutes(2);
        let state = FanState::with_timestamp("fan_1".to_string(), 0.5, 1000, past, false).unwrap();

        let age = state.age_seconds();
        assert!((115..=125).contains(&age)); // ~120 seconds ± 5 for test execution time
    }

    #[test]
    fn test_display() {
        let state = FanState::new("intake_fan".to_string(), 0.75, 25000, true).unwrap();
        let display = format!("{state}");
        assert!(display.contains("intake_fan"));
        assert!(display.contains("75.0%"));
        assert!(display.contains("25000Hz"));
        assert!(display.contains("MANUAL"));
    }

    #[test]
    fn test_serialization() {
        let state = FanState::new("fan_1".to_string(), 0.5, 1000, false).unwrap();

        // Test serialization
        let json = serde_json::to_string(&state).unwrap();
        assert!(json.contains("fan_1"));
        assert!(json.contains("0.5"));
        assert!(json.contains("1000"));

        // Test deserialization
        let deserialized: FanState = serde_json::from_str(&json).unwrap();
        assert_eq!(state, deserialized);
    }
}
