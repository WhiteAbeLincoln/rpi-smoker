use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(skip)]
    pub loaded_from: Option<PathBuf>,
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
    pub config_backup_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareConfig {
    pub ads1115: ADS1115Config,
    pub mock_mode: bool, // For development/testing
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
    pub fn to_volts(self) -> f64 {
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

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Default, Clone, Copy)]
#[repr(u8)]
pub enum ADSChannel {
    #[default]
    A0 = 0,
    A1 = 1,
    A2 = 2,
    A3 = 3,
}

// TODO: convert to DataRate16Bit from ads1x1x
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Default, Clone, Copy)]
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
    #[serde(skip)]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanConfig {
    pub pwm_pin: u8,
    pub frequency_hz: u32,
    pub duty_cycle_formulas: Vec<ConditionalFormula>,
    // not deserialized, this is the name in the hashmap for easy reference
    #[serde(skip)]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalFormula {
    pub when: Option<String>, // Optional condition
    pub value: String,        // Formula to evaluate
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlarmConfig {
    // not deserialized, this is the name in the hashmap for easy reference
    #[serde(skip)]
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

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
            cors_origins: vec!["*".to_string()],
            request_timeout_seconds: 30,
            config_backup_dir: "config_backup".into(),
        }
    }
}

impl Default for HardwareConfig {
    fn default() -> Self {
        Self {
            ads1115: ADS1115Config::default(),
            mock_mode: true,
        }
    }
}

impl Default for ADS1115Config {
    fn default() -> Self {
        Self {
            i2c_bus: Some(1),
            address: Some(0x48),
            gain: ADSGain::default(),
            data_rate: ADSDataRate::default(),
            poll_interval_ms: 1000,
        }
    }
}

impl Default for DataRetentionConfig {
    fn default() -> Self {
        Self {
            max_age_hours: 24 * 7, // 1 week
            max_memory_mb: 1024,   // 1GB
            cleanup_threshold_percent: 0.8,
            cleanup_percentage: 0.2,
        }
    }
}
