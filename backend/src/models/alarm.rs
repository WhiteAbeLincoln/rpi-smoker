use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::validation::{
    Validate, ValidationError, validate_id_format, validate_timestamp_not_future,
};

/// Represents an alarm event that can be triggered by various conditions
/// Note: condition and message are available via AlarmConfig using alarm_id
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlarmEvent {
    /// Unique identifier for this alarm (e.g., "high_temp_alarm_1")
    pub alarm_id: String,
    /// Whether the alarm is currently triggered
    pub triggered: bool,
    /// When this alarm event occurred
    pub timestamp: DateTime<Utc>,
    /// Whether the alarm has been acknowledged by a user
    pub acknowledged: bool,
    /// Number of times this alarm has been triggered in the current session
    pub trigger_count: u32,
}

impl AlarmEvent {
    /// Create a new alarm event with validation
    pub fn new(alarm_id: String, triggered: bool) -> Result<Self, ValidationError> {
        let event = Self {
            alarm_id,
            triggered,
            timestamp: Utc::now(),
            acknowledged: false,
            trigger_count: if triggered { 1 } else { 0 },
        };
        event.validate()?;
        Ok(event)
    }

    /// Create an alarm event with a specific timestamp
    pub fn with_timestamp(
        alarm_id: String,
        triggered: bool,
        timestamp: DateTime<Utc>,
    ) -> Result<Self, ValidationError> {
        let event = Self {
            alarm_id,
            triggered,
            timestamp,
            acknowledged: false,
            trigger_count: if triggered { 1 } else { 0 },
        };
        event.validate()?;
        Ok(event)
    }

    /// Create a triggered alarm event
    pub fn triggered(alarm_id: String) -> Result<Self, ValidationError> {
        Self::new(alarm_id, true)
    }

    /// Create a cleared alarm event (not triggered)
    pub fn cleared(alarm_id: String) -> Result<Self, ValidationError> {
        Self::new(alarm_id, false)
    }

    /// Acknowledge this alarm
    pub fn acknowledge(&mut self) {
        self.acknowledged = true;
    }

    /// Create a new alarm event with acknowledged status
    pub fn with_acknowledged(&self, acknowledged: bool) -> Self {
        let mut event = self.clone();
        event.acknowledged = acknowledged;
        event
    }

    /// Trigger this alarm (increment trigger count)
    pub fn trigger(&mut self) {
        self.triggered = true;
        self.trigger_count += 1;
        self.timestamp = Utc::now();
        self.acknowledged = false; // Reset acknowledgment when retriggered
    }

    /// Clear this alarm
    pub fn clear(&mut self) {
        self.triggered = false;
        self.timestamp = Utc::now();
        // Don't reset trigger_count or acknowledged status when clearing
    }

    /// Check if this alarm needs attention (triggered and not acknowledged)
    pub fn needs_attention(&self) -> bool {
        self.triggered && !self.acknowledged
    }

    /// Get the age of this alarm event in seconds
    pub fn age_seconds(&self) -> i64 {
        (Utc::now() - self.timestamp).num_seconds()
    }

    /// Get the severity level based on trigger count
    pub fn severity_level(&self) -> AlarmSeverity {
        match self.trigger_count {
            0 => AlarmSeverity::Info,
            1..=2 => AlarmSeverity::Warning,
            3..=5 => AlarmSeverity::High,
            _ => AlarmSeverity::Critical,
        }
    }

    /// Create a new alarm event with incremented trigger count
    pub fn with_incremented_trigger_count(&self) -> Self {
        let mut event = self.clone();
        event.trigger_count += 1;
        event.timestamp = Utc::now();
        event
    }
}

/// Alarm severity levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlarmSeverity {
    Info,
    Warning,
    High,
    Critical,
}

impl std::fmt::Display for AlarmSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlarmSeverity::Info => write!(f, "INFO"),
            AlarmSeverity::Warning => write!(f, "WARNING"),
            AlarmSeverity::High => write!(f, "HIGH"),
            AlarmSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl Validate for AlarmEvent {
    fn validate(&self) -> Result<(), ValidationError> {
        // Validate alarm ID format
        validate_id_format("alarm_id", &self.alarm_id)?;

        // Validate timestamp is not in the future
        validate_timestamp_not_future("timestamp", &self.timestamp)?;

        Ok(())
    }
}

impl std::fmt::Display for AlarmEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.triggered {
            if self.acknowledged {
                "TRIGGERED (ACK)"
            } else {
                "TRIGGERED"
            }
        } else {
            "CLEARED"
        };

        write!(
            f,
            "Alarm {} [{}] at {} (count: {})",
            self.alarm_id,
            status,
            self.timestamp.format("%H:%M:%S"),
            self.trigger_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_alarm_event_new() {
        let event = AlarmEvent::new("high_temp_alarm".to_string(), true);
        assert!(event.is_ok());

        let event = event.unwrap();
        assert_eq!(event.alarm_id, "high_temp_alarm");
        assert!(event.triggered);
        assert_eq!(event.trigger_count, 1);
        assert!(!event.acknowledged);
    }

    #[test]
    fn test_alarm_event_validation() {
        // Valid event
        assert!(AlarmEvent::new("alarm_1".to_string(), true,).is_ok());

        // Invalid alarm ID
        assert!(AlarmEvent::new("".to_string(), true,).is_err());
    }

    #[test]
    fn test_alarm_event_convenience_constructors() {
        // Test triggered
        let triggered = AlarmEvent::triggered("alarm_1".to_string()).unwrap();
        assert!(triggered.triggered);
        assert_eq!(triggered.trigger_count, 1);

        // Test cleared
        let cleared = AlarmEvent::cleared("alarm_1".to_string()).unwrap();
        assert!(!cleared.triggered);
        assert_eq!(cleared.trigger_count, 0);
    }

    #[test]
    fn test_alarm_event_acknowledge() {
        let mut event = AlarmEvent::triggered("alarm_1".to_string()).unwrap();

        assert!(!event.acknowledged);
        assert!(event.needs_attention());

        event.acknowledge();
        assert!(event.acknowledged);
        assert!(!event.needs_attention());
    }

    #[test]
    fn test_alarm_event_trigger_and_clear() {
        let mut event = AlarmEvent::cleared("alarm_1".to_string()).unwrap();

        assert!(!event.triggered);
        assert_eq!(event.trigger_count, 0);

        // Trigger the alarm
        event.trigger();
        assert!(event.triggered);
        assert_eq!(event.trigger_count, 1);
        assert!(!event.acknowledged);

        // Acknowledge and trigger again
        event.acknowledge();
        event.trigger();
        assert!(event.triggered);
        assert_eq!(event.trigger_count, 2);
        assert!(!event.acknowledged); // Should reset on retrigger

        // Clear the alarm
        event.clear();
        assert!(!event.triggered);
        assert_eq!(event.trigger_count, 2); // Count preserved
        assert!(!event.acknowledged); // Acknowledgment preserved
    }

    #[test]
    fn test_alarm_severity_levels() {
        let mut event = AlarmEvent::cleared("alarm_1".to_string()).unwrap();

        // Test severity progression
        assert_eq!(event.severity_level(), AlarmSeverity::Info);

        event.trigger_count = 1;
        assert_eq!(event.severity_level(), AlarmSeverity::Warning);

        event.trigger_count = 3;
        assert_eq!(event.severity_level(), AlarmSeverity::High);

        event.trigger_count = 10;
        assert_eq!(event.severity_level(), AlarmSeverity::Critical);
    }

    #[test]
    fn test_alarm_with_future_timestamp() {
        let future = Utc::now() + Duration::hours(1);
        let event = AlarmEvent::with_timestamp("alarm_1".to_string(), true, future);
        assert!(event.is_err());
    }

    #[test]
    fn test_age_seconds() {
        let past = Utc::now() - Duration::minutes(3);
        let event = AlarmEvent::with_timestamp("alarm_1".to_string(), true, past).unwrap();

        let age = event.age_seconds();
        assert!((175..=185).contains(&age)); // ~180 seconds ± 5 for test execution time
    }

    #[test]
    fn test_display() {
        let mut event = AlarmEvent::triggered("high_temp_alarm".to_string()).unwrap();

        let display = format!("{event}");
        assert!(display.contains("high_temp_alarm"));
        assert!(display.contains("TRIGGERED"));
        assert!(display.contains("count: 1"));

        event.acknowledge();
        let display = format!("{event}");
        assert!(display.contains("TRIGGERED (ACK)"));
    }

    #[test]
    fn test_serialization() {
        let event = AlarmEvent::triggered("alarm_1".to_string()).unwrap();

        // Test serialization
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("alarm_1"));

        // Test deserialization
        let deserialized: AlarmEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }
}
