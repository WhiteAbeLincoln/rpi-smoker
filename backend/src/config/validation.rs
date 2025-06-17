use std::ffi::OsStr;

use super::error::{ConfigError, ConfigResult};
use super::models::*;

pub trait ConfigValidator {
    fn validate(&self) -> ConfigResult<()>;
}

impl ConfigValidator for AppConfig {
    fn validate(&self) -> ConfigResult<()> {
        self.server.validate()?;
        self.hardware.validate()?;
        self.data_retention.validate()?;

        // Validate temp sensors
        for (name, sensor) in &self.temp_sensors {
            validate_sensor_name(name)?;
            sensor.validate()?;
        }

        // Validate fans
        for (name, fan) in &self.fans {
            validate_fan_name(name)?;
            fan.validate()?;
        }

        // Validate alarms
        for (name, alarm) in &self.alarms {
            validate_alarm_name(name)?;
            alarm.validate()?;
        }

        Ok(())
    }
}

impl ConfigValidator for ServerConfig {
    fn validate(&self) -> ConfigResult<()> {
        validate_port(self.port)?;
        validate_timeout(self.request_timeout_seconds)?;
        validate_host(&self.host)?;
        validate_backup_dir(self.config_backup_dir.as_os_str())?;
        Ok(())
    }
}

impl ConfigValidator for HardwareConfig {
    fn validate(&self) -> ConfigResult<()> {
        self.ads1115.validate()
    }
}

impl ConfigValidator for ADS1115Config {
    fn validate(&self) -> ConfigResult<()> {
        validate_poll_interval(self.poll_interval_ms)?;

        if let Some(address) = self.address {
            validate_i2c_address(address)?;
        }

        if let Some(bus) = self.i2c_bus {
            validate_i2c_bus(bus)?;
        }

        Ok(())
    }
}

impl ConfigValidator for TempSensorConfig {
    fn validate(&self) -> ConfigResult<()> {
        validate_formula(&self.conversion_formula)?;
        Ok(())
    }
}

impl ConfigValidator for FanConfig {
    fn validate(&self) -> ConfigResult<()> {
        validate_pwm_pin(self.pwm_pin)?;
        validate_frequency(self.frequency_hz)?;

        for formula in &self.duty_cycle_formulas {
            formula.validate()?;
        }

        Ok(())
    }
}

impl ConfigValidator for ConditionalFormula {
    fn validate(&self) -> ConfigResult<()> {
        validate_formula(&self.value)?;

        if let Some(ref condition) = self.when {
            validate_formula(condition)?;
        }

        Ok(())
    }
}

impl ConfigValidator for AlarmConfig {
    fn validate(&self) -> ConfigResult<()> {
        validate_formula(&self.condition)?;
        validate_alarm_message(&self.message)?;
        validate_alarm_actions(&self.actions)?;
        Ok(())
    }
}

impl ConfigValidator for DataRetentionConfig {
    fn validate(&self) -> ConfigResult<()> {
        validate_memory_limit(self.max_memory_mb)?;
        validate_percentage(self.cleanup_threshold_percent, "cleanup_threshold_percent")?;
        validate_percentage(self.cleanup_percentage, "cleanup_percentage")?;
        validate_age_hours(self.max_age_hours)?;
        Ok(())
    }
}

// Validation functions

fn validate_port(port: u16) -> ConfigResult<()> {
    if port < 1024 {
        return Err(ConfigError::ValidationFailed {
            field: "port".to_string(),
            message: "Port must be between 1024 and 65535".to_string(),
        });
    }
    Ok(())
}

fn validate_timeout(timeout: u64) -> ConfigResult<()> {
    if !(1..=3600).contains(&timeout) {
        return Err(ConfigError::ValidationFailed {
            field: "request_timeout_seconds".to_string(),
            message: "Timeout must be between 1 and 3600 seconds".to_string(),
        });
    }
    Ok(())
}

fn validate_host(host: &str) -> ConfigResult<()> {
    if host.is_empty() {
        return Err(ConfigError::ValidationFailed {
            field: "host".to_string(),
            message: "Host cannot be empty".to_string(),
        });
    }
    Ok(())
}

fn validate_backup_dir(dir: &OsStr) -> ConfigResult<()> {
    if dir.is_empty() {
        return Err(ConfigError::ValidationFailed {
            field: "config_backup_dir".to_string(),
            message: "Backup directory cannot be empty".to_string(),
        });
    }
    Ok(())
}

fn validate_poll_interval(interval: u64) -> ConfigResult<()> {
    if !(250..=60000).contains(&interval) {
        return Err(ConfigError::ValidationFailed {
            field: "poll_interval_ms".to_string(),
            message: "Poll interval must be between 250ms and 60000ms".to_string(),
        });
    }
    Ok(())
}

fn validate_i2c_address(address: u16) -> ConfigResult<()> {
    if address > 0xFF {
        return Err(ConfigError::ValidationFailed {
            field: "i2c_address".to_string(),
            message: "I2C address must be between 0x00 and 0xFF".to_string(),
        });
    }
    Ok(())
}

fn validate_i2c_bus(bus: u8) -> ConfigResult<()> {
    if bus > 10 {
        return Err(ConfigError::ValidationFailed {
            field: "i2c_bus".to_string(),
            message: "I2C bus number must be reasonable (0-10)".to_string(),
        });
    }
    Ok(())
}

fn validate_pwm_pin(pin: u8) -> ConfigResult<()> {
    // Valid GPIO pins for Raspberry Pi (simplified check)
    const VALID_GPIO_PINS: &[u8] = &[
        2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
        27,
    ];

    if !VALID_GPIO_PINS.contains(&pin) {
        return Err(ConfigError::ValidationFailed {
            field: "pwm_pin".to_string(),
            message: format!("GPIO pin {pin} is not valid for Raspberry Pi"),
        });
    }
    Ok(())
}

fn validate_frequency(freq: u32) -> ConfigResult<()> {
    if !(1..=100000).contains(&freq) {
        return Err(ConfigError::ValidationFailed {
            field: "frequency_hz".to_string(),
            message: "PWM frequency must be between 1Hz and 100kHz".to_string(),
        });
    }
    Ok(())
}

fn validate_formula(formula: &str) -> ConfigResult<()> {
    if formula.is_empty() {
        return Err(ConfigError::ValidationFailed {
            field: "formula".to_string(),
            message: "Formula cannot be empty".to_string(),
        });
    }
    // Basic validation - more sophisticated validation would require a formula engine
    if formula.len() > 1000 {
        return Err(ConfigError::ValidationFailed {
            field: "formula".to_string(),
            message: "Formula is too long (max 1000 characters)".to_string(),
        });
    }
    Ok(())
}

fn validate_percentage(value: f64, field: &str) -> ConfigResult<()> {
    if !(0.0..=1.0).contains(&value) {
        return Err(ConfigError::ValidationFailed {
            field: field.to_string(),
            message: "Percentage value must be between 0.0 and 1.0".to_string(),
        });
    }
    Ok(())
}

fn validate_memory_limit(mb: u64) -> ConfigResult<()> {
    if !(10..=4096).contains(&mb) {
        return Err(ConfigError::ValidationFailed {
            field: "max_memory_mb".to_string(),
            message: "Memory limit must be between 10MB and 4GB".to_string(),
        });
    }
    Ok(())
}

fn validate_age_hours(hours: u64) -> ConfigResult<()> {
    if !(1..=8760).contains(&hours) {
        // 1 year
        return Err(ConfigError::ValidationFailed {
            field: "max_age_hours".to_string(),
            message: "Age limit must be between 1 hour and 1 year".to_string(),
        });
    }
    Ok(())
}

fn validate_sensor_name(name: &str) -> ConfigResult<()> {
    if name.is_empty() || name.len() > 50 {
        return Err(ConfigError::ValidationFailed {
            field: "sensor_name".to_string(),
            message: "Sensor name must be between 1 and 50 characters".to_string(),
        });
    }
    Ok(())
}

fn validate_fan_name(name: &str) -> ConfigResult<()> {
    if name.is_empty() || name.len() > 50 {
        return Err(ConfigError::ValidationFailed {
            field: "fan_name".to_string(),
            message: "Fan name must be between 1 and 50 characters".to_string(),
        });
    }
    Ok(())
}

fn validate_alarm_name(name: &str) -> ConfigResult<()> {
    if name.is_empty() || name.len() > 50 {
        return Err(ConfigError::ValidationFailed {
            field: "alarm_name".to_string(),
            message: "Alarm name must be between 1 and 50 characters".to_string(),
        });
    }
    Ok(())
}

fn validate_alarm_message(message: &str) -> ConfigResult<()> {
    if message.is_empty() || message.len() > 500 {
        return Err(ConfigError::ValidationFailed {
            field: "alarm_message".to_string(),
            message: "Alarm message must be between 1 and 500 characters".to_string(),
        });
    }
    Ok(())
}

fn validate_alarm_actions(actions: &[String]) -> ConfigResult<()> {
    if actions.is_empty() {
        return Err(ConfigError::ValidationFailed {
            field: "alarm_actions".to_string(),
            message: "At least one alarm action must be specified".to_string(),
        });
    }

    for action in actions {
        if action.is_empty() || action.len() > 100 {
            return Err(ConfigError::ValidationFailed {
                field: "alarm_action".to_string(),
                message: "Alarm action must be between 1 and 100 characters".to_string(),
            });
        }
    }
    Ok(())
}
