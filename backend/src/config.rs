use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub hardware: HardwareConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
    pub request_timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HardwareConfig {
    pub enable_gpio: bool,
    pub temperature_sensor_pin: Option<u8>,
    pub fan_control_pin: Option<u8>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            host: "0.0.0.0".to_string(),
            cors_origins: vec!["*".to_string()],
            request_timeout_seconds: 30,
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // For now, return default config
        // TODO: Load from file, environment variables, etc.
        Ok(Self::default())
    }
}
