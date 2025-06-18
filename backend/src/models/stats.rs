use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{FanState, SensorReading};

/// Overall system statistics and data summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataStats {
    /// Total number of sensor readings stored
    pub total_readings: usize,
    /// Current memory usage in bytes
    pub memory_usage_bytes: usize,
    /// Memory limit in bytes before cleanup
    pub memory_limit_bytes: usize,
    /// Timestamp of the oldest reading in storage
    pub oldest_reading: Option<DateTime<Utc>>,
    /// Timestamp of the newest reading in storage
    pub newest_reading: Option<DateTime<Utc>>,
    /// How long the current session has been running
    pub session_duration_seconds: u64,
    /// Number of cleanup events (old data removal)
    pub cleanup_events: u64,
    /// Statistics for each sensor
    pub sensors_by_id: HashMap<String, SensorStats>,
    /// Statistics for each fan
    pub fans_by_id: HashMap<String, FanStats>,
}

impl DataStats {
    /// Create new empty data statistics
    pub fn new(memory_limit_bytes: usize) -> Self {
        Self {
            total_readings: 0,
            memory_usage_bytes: 0,
            memory_limit_bytes,
            oldest_reading: None,
            newest_reading: None,
            session_duration_seconds: 0,
            cleanup_events: 0,
            sensors_by_id: HashMap::new(),
            fans_by_id: HashMap::new(),
        }
    }

    /// Check if memory usage is approaching the limit
    pub fn is_memory_usage_high(&self) -> bool {
        self.memory_usage_bytes as f64 / self.memory_limit_bytes as f64 > 0.8
    }

    /// Check if memory usage has exceeded the limit
    pub fn is_memory_limit_exceeded(&self) -> bool {
        self.memory_usage_bytes > self.memory_limit_bytes
    }

    /// Get memory usage as a percentage
    pub fn memory_usage_percent(&self) -> f64 {
        if self.memory_limit_bytes == 0 {
            0.0
        } else {
            (self.memory_usage_bytes as f64 / self.memory_limit_bytes as f64) * 100.0
        }
    }

    /// Get the time span of stored data in seconds
    pub fn data_timespan_seconds(&self) -> Option<i64> {
        match (self.oldest_reading, self.newest_reading) {
            (Some(oldest), Some(newest)) => Some((newest - oldest).num_seconds()),
            _ => None,
        }
    }

    /// Get the number of active sensors (sensors with readings)
    pub fn active_sensor_count(&self) -> usize {
        self.sensors_by_id
            .values()
            .filter(|stats| stats.reading_count > 0)
            .count()
    }

    /// Get the number of active fans (fans with state changes)
    pub fn active_fan_count(&self) -> usize {
        self.fans_by_id
            .values()
            .filter(|stats| stats.state_changes > 0)
            .count()
    }

    /// Get average readings per sensor
    pub fn average_readings_per_sensor(&self) -> f64 {
        let active_sensors = self.active_sensor_count();
        if active_sensors == 0 {
            0.0
        } else {
            self.total_readings as f64 / active_sensors as f64
        }
    }
}

/// Statistics for a specific sensor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorStats {
    /// Number of readings recorded for this sensor
    pub reading_count: usize,
    /// Most recent reading from this sensor
    pub last_reading: Option<SensorReading>,
    /// Average temperature over all readings
    pub avg_temperature: f64,
    /// Minimum temperature recorded
    pub min_temperature: f64,
    /// Maximum temperature recorded
    pub max_temperature: f64,
    /// Number of errors or invalid readings
    pub error_count: u32,
}

impl SensorStats {
    /// Create new sensor statistics
    pub fn new() -> Self {
        Self {
            reading_count: 0,
            last_reading: None,
            avg_temperature: 0.0,
            min_temperature: f64::INFINITY,
            max_temperature: f64::NEG_INFINITY,
            error_count: 0,
        }
    }

    /// Update statistics with a new reading
    pub fn update_with_reading(&mut self, reading: &SensorReading) {
        let temp = reading.temperature_celsius;

        // Update count and last reading
        self.reading_count += 1;
        self.last_reading = Some(reading.clone());

        // Update temperature statistics
        if self.reading_count == 1 {
            // First reading
            self.avg_temperature = temp;
            self.min_temperature = temp;
            self.max_temperature = temp;
        } else {
            // Update running average
            self.avg_temperature = ((self.avg_temperature * (self.reading_count - 1) as f64)
                + temp)
                / self.reading_count as f64;

            // Update min/max
            self.min_temperature = self.min_temperature.min(temp);
            self.max_temperature = self.max_temperature.max(temp);
        }
    }

    /// Record an error for this sensor
    pub fn record_error(&mut self) {
        self.error_count += 1;
    }

    /// Get the temperature range (max - min)
    pub fn temperature_range(&self) -> f64 {
        if self.reading_count == 0 {
            0.0
        } else {
            self.max_temperature - self.min_temperature
        }
    }

    /// Check if this sensor has recent data (within the last 60 seconds)
    pub fn has_recent_data(&self) -> bool {
        match &self.last_reading {
            Some(reading) => reading.age_seconds() < 60,
            None => false,
        }
    }

    /// Get error rate as a percentage
    pub fn error_rate_percent(&self) -> f64 {
        if self.reading_count == 0 {
            0.0
        } else {
            (self.error_count as f64 / (self.reading_count + self.error_count as usize) as f64)
                * 100.0
        }
    }
}

impl Default for SensorStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for a specific fan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanStats {
    /// Number of state changes for this fan
    pub state_changes: usize,
    /// Current state of the fan
    pub current_state: Option<FanState>,
    /// Average duty cycle over all state changes
    pub avg_duty_cycle: f64,
    /// Total runtime in seconds (when duty cycle > 0)
    pub total_runtime_seconds: u64,
}

impl FanStats {
    /// Create new fan statistics
    pub fn new() -> Self {
        Self {
            state_changes: 0,
            current_state: None,
            avg_duty_cycle: 0.0,
            total_runtime_seconds: 0,
        }
    }

    /// Update statistics with a new fan state
    pub fn update_with_state(&mut self, state: &FanState) {
        // Calculate runtime since last state change if fan was running
        if let Some(ref last_state) = self.current_state
            && last_state.is_running()
        {
            let runtime_delta = (state.timestamp - last_state.timestamp).num_seconds();
            if runtime_delta > 0 {
                self.total_runtime_seconds += runtime_delta as u64;
            }
        }

        // Update count and current state
        self.state_changes += 1;
        self.current_state = Some(state.clone());

        // Update average duty cycle
        if self.state_changes == 1 {
            self.avg_duty_cycle = state.duty_cycle;
        } else {
            self.avg_duty_cycle = ((self.avg_duty_cycle * (self.state_changes - 1) as f64)
                + state.duty_cycle)
                / self.state_changes as f64;
        }
    }

    /// Check if this fan is currently running
    pub fn is_currently_running(&self) -> bool {
        match &self.current_state {
            Some(state) => state.is_running(),
            None => false,
        }
    }

    /// Check if this fan has recent activity (state change within last 5 minutes)
    pub fn has_recent_activity(&self) -> bool {
        match &self.current_state {
            Some(state) => state.age_seconds() < 300, // 5 minutes
            None => false,
        }
    }

    /// Get runtime as a percentage of total session time
    pub fn runtime_percentage(&self, session_duration_seconds: u64) -> f64 {
        if session_duration_seconds == 0 {
            0.0
        } else {
            (self.total_runtime_seconds as f64 / session_duration_seconds as f64) * 100.0
        }
    }

    /// Get average duty cycle as a percentage
    pub fn avg_duty_cycle_percent(&self) -> f64 {
        self.avg_duty_cycle * 100.0
    }
}

impl Default for FanStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_data_stats_new() {
        let stats = DataStats::new(1024 * 1024); // 1MB limit
        assert_eq!(stats.total_readings, 0);
        assert_eq!(stats.memory_limit_bytes, 1024 * 1024);
        assert!(!stats.is_memory_usage_high());
        assert!(!stats.is_memory_limit_exceeded());
    }

    #[test]
    fn test_data_stats_memory_checks() {
        let mut stats = DataStats::new(1000);

        stats.memory_usage_bytes = 850; // 85%
        assert!(stats.is_memory_usage_high());
        assert!(!stats.is_memory_limit_exceeded());
        assert_eq!(stats.memory_usage_percent(), 85.0);

        stats.memory_usage_bytes = 1100; // 110%
        assert!(stats.is_memory_limit_exceeded());
        assert!((stats.memory_usage_percent() - 110.0).abs() < 0.01); // Use floating point comparison
    }

    #[test]
    fn test_data_stats_timespan() {
        let mut stats = DataStats::new(1000);

        assert!(stats.data_timespan_seconds().is_none());

        let now = Utc::now();
        let past = now - Duration::hours(2);

        stats.oldest_reading = Some(past);
        stats.newest_reading = Some(now);

        let timespan = stats.data_timespan_seconds().unwrap();
        assert!((7195..=7205).contains(&timespan)); // ~7200 seconds ± 5
    }

    #[test]
    fn test_sensor_stats_updates() {
        let mut stats = SensorStats::new();

        // Test first reading
        let reading1 = SensorReading::new("temp_1".to_string(), 3.3, 25.0, None).unwrap();

        stats.update_with_reading(&reading1);
        assert_eq!(stats.reading_count, 1);
        assert_eq!(stats.avg_temperature, 25.0);
        assert_eq!(stats.min_temperature, 25.0);
        assert_eq!(stats.max_temperature, 25.0);

        // Test second reading
        let reading2 = SensorReading::new("temp_1".to_string(), 3.4, 35.0, None).unwrap();

        stats.update_with_reading(&reading2);
        assert_eq!(stats.reading_count, 2);
        assert_eq!(stats.avg_temperature, 30.0); // (25 + 35) / 2
        assert_eq!(stats.min_temperature, 25.0);
        assert_eq!(stats.max_temperature, 35.0);
        assert_eq!(stats.temperature_range(), 10.0);
    }

    #[test]
    fn test_sensor_stats_errors() {
        let mut stats = SensorStats::new();

        // Add some readings and errors
        let reading = SensorReading::new("temp_1".to_string(), 3.3, 25.0, None).unwrap();
        stats.update_with_reading(&reading);
        stats.record_error();
        stats.record_error();

        assert_eq!(stats.error_count, 2);
        // Error rate: 2 errors / (1 reading + 2 errors) = 66.67%
        assert!((stats.error_rate_percent() - 66.67).abs() < 0.01);
    }

    #[test]
    fn test_fan_stats_updates() {
        let mut stats = FanStats::new();

        // Test first state
        let state1 = FanState::new("fan_1".to_string(), 0.5, 1000, false).unwrap();

        stats.update_with_state(&state1);
        assert_eq!(stats.state_changes, 1);
        assert_eq!(stats.avg_duty_cycle, 0.5);
        assert!(stats.is_currently_running());

        // Test second state (after some time)
        let mut state2 = FanState::new("fan_1".to_string(), 0.8, 1000, false).unwrap();
        state2.timestamp = state1.timestamp + Duration::seconds(60);

        stats.update_with_state(&state2);
        assert_eq!(stats.state_changes, 2);
        assert_eq!(stats.avg_duty_cycle, 0.65); // (0.5 + 0.8) / 2
        assert_eq!(stats.total_runtime_seconds, 60); // First state was running for 60 seconds
    }

    #[test]
    fn test_fan_stats_runtime_percentage() {
        let mut stats = FanStats::new();
        stats.total_runtime_seconds = 300; // 5 minutes

        let percentage = stats.runtime_percentage(1200); // 20 minutes session
        assert_eq!(percentage, 25.0); // 5/20 = 25%
    }

    #[test]
    fn test_serialization() {
        let stats = DataStats::new(1000);

        // Test serialization
        let json = serde_json::to_string(&stats).unwrap();
        assert!(json.contains("total_readings"));
        assert!(json.contains("memory_limit_bytes"));

        // Test deserialization
        let deserialized: DataStats = serde_json::from_str(&json).unwrap();
        assert_eq!(stats.total_readings, deserialized.total_readings);
        assert_eq!(stats.memory_limit_bytes, deserialized.memory_limit_bytes);
    }
}
