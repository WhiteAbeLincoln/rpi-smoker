use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{AlarmEvent, DataStats, FanState, SensorReading};

/// Generic API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Whether the request was successful
    pub success: bool,
    /// Response data (if successful)
    pub data: Option<T>,
    /// Error message (if unsuccessful)
    pub error: Option<String>,
    /// Timestamp when the response was generated
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    /// Create a successful response with data
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }

    /// Create an error response
    pub fn error(error_message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error_message),
            timestamp: Utc::now(),
        }
    }

    /// Create an error response from an error type
    pub fn from_error<E: std::fmt::Display>(error: E) -> Self {
        Self::error(error.to_string())
    }
}

/// Response containing sensor readings
#[derive(Debug, Serialize, Deserialize)]
pub struct SensorsResponse {
    /// Map of sensor ID to latest sensor reading
    pub sensors: HashMap<String, SensorReading>,
    /// When this response was generated
    pub timestamp: DateTime<Utc>,
}

impl SensorsResponse {
    /// Create a new sensors response
    pub fn new(sensors: HashMap<String, SensorReading>) -> Self {
        Self {
            sensors,
            timestamp: Utc::now(),
        }
    }

    /// Create an empty sensors response
    pub fn empty() -> Self {
        Self::new(HashMap::new())
    }

    /// Get the number of sensors in the response
    pub fn sensor_count(&self) -> usize {
        self.sensors.len()
    }

    /// Check if a specific sensor is included
    pub fn has_sensor(&self, sensor_id: &str) -> bool {
        self.sensors.contains_key(sensor_id)
    }

    /// Get the average temperature across all sensors
    pub fn average_temperature(&self) -> Option<f64> {
        if self.sensors.is_empty() {
            return None;
        }

        let total: f64 = self
            .sensors
            .values()
            .map(|reading| reading.temperature_celsius)
            .sum();
        Some(total / self.sensors.len() as f64)
    }

    /// Get the highest temperature reading
    pub fn max_temperature(&self) -> Option<f64> {
        self.sensors
            .values()
            .map(|reading| reading.temperature_celsius)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Get the lowest temperature reading
    pub fn min_temperature(&self) -> Option<f64> {
        self.sensors
            .values()
            .map(|reading| reading.temperature_celsius)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }
}

/// Response containing fan states
#[derive(Debug, Serialize, Deserialize)]
pub struct FansResponse {
    /// Map of fan ID to current fan state
    pub fans: HashMap<String, FanState>,
    /// When this response was generated
    pub timestamp: DateTime<Utc>,
}

impl FansResponse {
    /// Create a new fans response
    pub fn new(fans: HashMap<String, FanState>) -> Self {
        Self {
            fans,
            timestamp: Utc::now(),
        }
    }

    /// Create an empty fans response
    pub fn empty() -> Self {
        Self::new(HashMap::new())
    }

    /// Get the number of fans in the response
    pub fn fan_count(&self) -> usize {
        self.fans.len()
    }

    /// Check if a specific fan is included
    pub fn has_fan(&self, fan_id: &str) -> bool {
        self.fans.contains_key(fan_id)
    }

    /// Get the number of running fans
    pub fn running_fan_count(&self) -> usize {
        self.fans
            .values()
            .filter(|state| state.is_running())
            .count()
    }

    /// Get the average duty cycle across all fans
    pub fn average_duty_cycle(&self) -> Option<f64> {
        if self.fans.is_empty() {
            return None;
        }

        let total: f64 = self.fans.values().map(|state| state.duty_cycle).sum();
        Some(total / self.fans.len() as f64)
    }

    /// Get fans that are manually overridden
    pub fn manual_override_fans(&self) -> Vec<&FanState> {
        self.fans
            .values()
            .filter(|state| state.is_manual_override)
            .collect()
    }
}

/// Response containing alarm events
#[derive(Debug, Serialize, Deserialize)]
pub struct AlarmsResponse {
    /// Map of alarm ID to alarm event
    pub alarms: HashMap<String, AlarmEvent>,
    /// Number of currently active/triggered alarms
    pub active_count: usize,
    /// When this response was generated
    pub timestamp: DateTime<Utc>,
}

impl AlarmsResponse {
    /// Create a new alarms response
    pub fn new(alarms: HashMap<String, AlarmEvent>) -> Self {
        let active_count = alarms.values().filter(|alarm| alarm.triggered).count();

        Self {
            alarms,
            active_count,
            timestamp: Utc::now(),
        }
    }

    /// Create an empty alarms response
    pub fn empty() -> Self {
        Self::new(HashMap::new())
    }

    /// Get the total number of alarms
    pub fn total_alarm_count(&self) -> usize {
        self.alarms.len()
    }

    /// Get alarms that need attention (triggered and not acknowledged)
    pub fn alarms_needing_attention(&self) -> Vec<&AlarmEvent> {
        self.alarms
            .values()
            .filter(|alarm| alarm.needs_attention())
            .collect()
    }

    /// Get the number of alarms needing attention
    pub fn attention_count(&self) -> usize {
        self.alarms_needing_attention().len()
    }

    /// Check if there are any critical alarms
    pub fn has_critical_alarms(&self) -> bool {
        self.alarms.values().any(|alarm| {
            alarm.triggered && alarm.severity_level() == super::AlarmSeverity::Critical
        })
    }

    /// Get alarms by severity level
    pub fn alarms_by_severity(&self, severity: super::AlarmSeverity) -> Vec<&AlarmEvent> {
        self.alarms
            .values()
            .filter(|alarm| alarm.severity_level() == severity)
            .collect()
    }
}

/// Response containing system statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct StatsResponse {
    /// System statistics and data summary
    pub stats: DataStats,
    /// When this response was generated
    pub timestamp: DateTime<Utc>,
}

impl StatsResponse {
    /// Create a new stats response
    pub fn new(stats: DataStats) -> Self {
        Self {
            stats,
            timestamp: Utc::now(),
        }
    }
}

/// Request to update fan state
#[derive(Debug, Deserialize)]
pub struct UpdateFanRequest {
    /// New duty cycle (0.0 to 1.0)
    pub duty_cycle: f64,
    /// PWM frequency in Hz (optional, uses current if not specified)
    pub pwm_frequency_hz: Option<u32>,
    /// Whether this is a manual override
    pub is_manual_override: bool,
}

/// Request to acknowledge an alarm
#[derive(Debug, Deserialize)]
pub struct AcknowledgeAlarmRequest {
    /// Whether to acknowledge (true) or unacknowledge (false) the alarm
    pub acknowledged: bool,
}

/// Request to configure system settings
#[derive(Debug, Deserialize)]
pub struct SystemConfigRequest {
    /// Memory limit in bytes
    pub memory_limit_bytes: Option<usize>,
    /// Data cleanup interval in seconds
    pub cleanup_interval_seconds: Option<u64>,
}

/// Health check response
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Service status
    pub status: String,
    /// Version information
    pub version: String,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// When this health check was performed
    pub timestamp: DateTime<Utc>,
}

impl HealthResponse {
    /// Create a healthy response
    pub fn healthy(version: String, uptime_seconds: u64) -> Self {
        Self {
            status: "healthy".to_string(),
            version,
            uptime_seconds,
            timestamp: Utc::now(),
        }
    }

    /// Create an unhealthy response
    pub fn unhealthy(version: String, uptime_seconds: u64) -> Self {
        Self {
            status: "unhealthy".to_string(),
            version,
            uptime_seconds,
            timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response() {
        // Test success response
        let success_response = ApiResponse::success("test data".to_string());
        assert!(success_response.success);
        assert_eq!(success_response.data, Some("test data".to_string()));
        assert!(success_response.error.is_none());

        // Test error response
        let error_response = ApiResponse::<String>::error("test error".to_string());
        assert!(!error_response.success);
        assert!(error_response.data.is_none());
        assert_eq!(error_response.error, Some("test error".to_string()));
    }

    #[test]
    fn test_sensors_response() {
        let mut sensors = HashMap::new();

        let reading1 = SensorReading::new("temp_1".to_string(), 3.3, 25.0, None).unwrap();
        let reading2 = SensorReading::new("temp_2".to_string(), 3.4, 35.0, None).unwrap();

        sensors.insert("temp_1".to_string(), reading1);
        sensors.insert("temp_2".to_string(), reading2);

        let response = SensorsResponse::new(sensors);

        assert_eq!(response.sensor_count(), 2);
        assert!(response.has_sensor("temp_1"));
        assert!(!response.has_sensor("temp_3"));
        assert_eq!(response.average_temperature(), Some(30.0));
        assert_eq!(response.max_temperature(), Some(35.0));
        assert_eq!(response.min_temperature(), Some(25.0));
    }

    #[test]
    fn test_fans_response() {
        let mut fans = HashMap::new();

        let state1 = FanState::new("fan_1".to_string(), 0.5, 1000, false).unwrap();
        let state2 = FanState::new("fan_2".to_string(), 0.8, 1000, true).unwrap();

        fans.insert("fan_1".to_string(), state1);
        fans.insert("fan_2".to_string(), state2);

        let response = FansResponse::new(fans);

        assert_eq!(response.fan_count(), 2);
        assert!(response.has_fan("fan_1"));
        assert_eq!(response.running_fan_count(), 2);
        assert_eq!(response.average_duty_cycle(), Some(0.65));
        assert_eq!(response.manual_override_fans().len(), 1);
    }

    #[test]
    fn test_alarms_response() {
        let mut alarms = HashMap::new();

        let alarm1 = AlarmEvent::triggered("alarm_1".to_string()).unwrap();

        let mut alarm2 = AlarmEvent::triggered("alarm_2".to_string()).unwrap();
        alarm2.trigger_count = 10; // Make it critical

        alarms.insert("alarm_1".to_string(), alarm1);
        alarms.insert("alarm_2".to_string(), alarm2);

        let response = AlarmsResponse::new(alarms);

        assert_eq!(response.total_alarm_count(), 2);
        assert_eq!(response.active_count, 2);
        assert_eq!(response.attention_count(), 2); // Both need attention
        assert!(response.has_critical_alarms());
    }

    #[test]
    fn test_health_response() {
        let healthy = HealthResponse::healthy("1.0.0".to_string(), 3600);
        assert_eq!(healthy.status, "healthy");
        assert_eq!(healthy.version, "1.0.0");
        assert_eq!(healthy.uptime_seconds, 3600);

        let unhealthy = HealthResponse::unhealthy("1.0.0".to_string(), 3600);
        assert_eq!(unhealthy.status, "unhealthy");
    }

    #[test]
    fn test_serialization() {
        let response = SensorsResponse::empty();

        // Test serialization
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("sensors"));
        assert!(json.contains("timestamp"));

        // Test deserialization
        let deserialized: SensorsResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(response.sensor_count(), deserialized.sensor_count());
    }
}
