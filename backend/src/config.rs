use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub hardware: HardwareConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareConfig {
    pub enable_gpio: bool,
    pub temperature_sensor_pin: Option<u8>,
    pub fan_control_pin: Option<u8>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
            },
            hardware: HardwareConfig {
                enable_gpio: false,
                temperature_sensor_pin: None,
                fan_control_pin: None,
            },
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
