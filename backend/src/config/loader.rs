use super::error::{ConfigError, ConfigResult};
use super::models::*;
use super::validation::ConfigValidator;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use serde::Serialize;
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};

// create a default config path once
lazy_static! {
    pub static ref DEFAULT_CONFIG_PATH: PathBuf = PathBuf::from("config.json");
}

impl AppConfig {
    /// Load configuration from file
    pub fn load(
        config_path: &Path,
        cli_overrides: Option<&CliOverrides>,
    ) -> ConfigResult<AppConfig> {
        let cfg_path = config_path.to_path_buf();
        let file = fs::File::open(config_path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                ConfigError::FileNotFound {
                    path: cfg_path.clone(),
                }
            } else {
                ConfigError::IoError { source: e }
            }
        })?;
        let mut config: AppConfig =
            serde_json::from_reader(file).map_err(|e| ConfigError::InvalidJson { source: e })?;

        // Set names for sensors, fans, and alarms based on HashMap keys
        config.populate_names();
        if let Some(overrides) = cli_overrides {
            config.apply_cli_overrides(overrides);
        }

        // Validate the configuration
        config.validate()?;

        config.loaded_from = Some(cfg_path);

        Ok(config)
    }

    pub fn get_file_path(&self) -> &Path {
        match self.loaded_from {
            Some(ref path) => path,
            // create a default path if not loaded from file
            None => &DEFAULT_CONFIG_PATH,
        }
    }

    /// Save configuration to file
    pub fn save(&self, path: Option<&Path>) -> ConfigResult<()> {
        // Validate before saving
        self.validate()?;

        // Use provided path or default
        let path = path.unwrap_or_else(|| self.get_file_path());

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?
        }

        serde_json::to_writer_pretty(fs::File::create(path)?, self)
            .map_err(|e| ConfigError::InvalidJson { source: e })
    }

    /// Update partial configuration (for API updates)
    pub fn update_partial(&mut self, _updates: serde_json::Value) -> ConfigResult<()> {
        // TODO: We need to update using a JSON patch
        panic!("Partial updates not implemented yet");
    }

    /// Backup current configuration
    pub fn backup_current(&self) -> ConfigResult<String> {
        // Create backup directory
        fs::create_dir_all(&self.server.config_backup_dir)?;

        // Generate backup filename with timestamp
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let backup_filename = format!("config_backup_{timestamp}.json");
        let backup_path = self.server.config_backup_dir.join(&backup_filename);

        // Save current config to backup
        self.save(Some(&backup_path))?;

        Ok(backup_filename)
    }

    /// List available backups
    pub fn list_backups(&self) -> ConfigResult<Vec<BackupInfo>> {
        let iter = match fs::read_dir(&self.server.config_backup_dir) {
            Ok(iter) => iter,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    return Ok(Vec::new());
                }
                return Err(e.into());
            }
        };

        let valid_files = iter.filter_map(|entry| -> Option<ConfigResult<PathBuf>> {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => return Some(Err(e.into())),
            };

            let path = entry.path();
            if !path.is_file() || path.extension().is_none_or(|ext| ext != "json") {
                return None;
            }
            Some(Ok(path))
        });

        let mut backups: Vec<BackupInfo> = valid_files
            .map(|path| -> ConfigResult<BackupInfo> {
                let path = path?;

                let metadata = fs::metadata(&path)?;
                let created = metadata
                    .created()
                    .map(DateTime::<Utc>::from)
                    .unwrap_or_else(|_| Utc::now());

                Ok(BackupInfo {
                    filename: path
                        .file_name()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or("".to_string()),
                    path: path.clone(),
                    created,
                    size: metadata.len(),
                })
            })
            .collect::<ConfigResult<Vec<_>>>()?;

        // Sort by creation time, newest first
        backups.sort_by(|a, b| b.created.cmp(&a.created));

        Ok(backups)
    }

    /// Restore configuration from backup
    pub fn restore_from_backup(&self, backup_filename: &str) -> ConfigResult<AppConfig> {
        // load backup
        let backup_path = self.server.config_backup_dir.join(backup_filename);
        let mut config = AppConfig::load(&backup_path, None)?;

        // backup current before overwriting
        self.backup_current()?;

        // change the loaded_from path to the current file and save
        if let Some(ref path) = self.loaded_from {
            config.loaded_from = Some(path.clone());
        }

        config.save(None)?;

        // return the restored config. The user will have to replace their reference to the current config
        // since there's not a great way to modify the existing reference (self)
        Ok(config)
    }

    /// Apply CLI overrides to configuration
    fn apply_cli_overrides(&mut self, cli_overrides: &CliOverrides) {
        if let Some(port) = cli_overrides.port {
            self.server.port = port;
        }

        if let Some(ref host) = cli_overrides.host {
            self.server.host = host.clone();
        }

        if let Some(timeout) = cli_overrides.timeout {
            self.server.request_timeout_seconds = timeout;
        }

        if let Some(mock_mode) = cli_overrides.mock_mode {
            self.hardware.mock_mode = mock_mode;
        }
    }

    /// Set names for hashmap-based configs based on their keys
    fn populate_names(&mut self) {
        for (name, sensor) in self.temp_sensors.iter_mut() {
            sensor.name = name.clone();
        }

        for (name, fan) in self.fans.iter_mut() {
            fan.name = name.clone();
        }

        for (name, alarm) in self.alarms.iter_mut() {
            alarm.name = name.clone();
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BackupInfo {
    pub filename: String,
    pub path: PathBuf,
    pub created: DateTime<Utc>,
    pub size: u64,
}

#[derive(Debug, Default)]
pub struct CliOverrides {
    pub port: Option<u16>,
    pub host: Option<String>,
    pub timeout: Option<u64>,
    pub mock_mode: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::TempDir;

    fn create_test_config(loaded_from: PathBuf, backup_dir: PathBuf) -> AppConfig {
        let mut temp_sensors = HashMap::new();
        temp_sensors.insert(
            "probe1".to_string(),
            TempSensorConfig {
                ads_channel: ADSChannel::A0,
                conversion_formula: "v * 100".to_string(),
                name: String::new(), // Will be populated by loader
            },
        );

        AppConfig {
            loaded_from: Some(loaded_from),
            server: ServerConfig {
                config_backup_dir: backup_dir,
                ..ServerConfig::default()
            },
            hardware: HardwareConfig::default(),
            temp_sensors,
            fans: HashMap::new(),
            alarms: HashMap::new(),
            data_retention: DataRetentionConfig::default(),
        }
    }

    #[test]
    fn test_load_save_config() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        let config_path = temp_dir.path().join("test_config.json");

        let original_config = create_test_config(config_path.clone(), backup_dir);

        // Save config
        original_config.save(None).unwrap();

        // Load config
        let loaded_config = AppConfig::load(&config_path, None).unwrap();

        // Check that name was populated
        assert_eq!(loaded_config.temp_sensors["probe1"].name, "probe1");
    }

    #[test]
    fn test_backup_and_restore() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.json");
        let backup_dir = temp_dir.path().join("backups");
        let mut config = create_test_config(config_path.clone(), backup_dir);

        // Save initial config
        config.save(None).unwrap();

        // Create backup
        config.backup_current().unwrap();

        // mutate and save again
        config.temp_sensors.clear();
        config.save(None).unwrap();

        // List backups
        let backups = config.list_backups().unwrap();
        assert_eq!(backups.len(), 1);

        // Restore from backup
        let restored_config = config.restore_from_backup(&backups[0].filename).unwrap();
        assert_eq!(restored_config.temp_sensors["probe1"].name, "probe1");

        // Check that the restored config was written to the original path
        let restored_config = AppConfig::load(&config_path, None).unwrap();
        assert_eq!(restored_config.temp_sensors["probe1"].name, "probe1");
    }

    #[test]
    fn test_cli_overrides() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.json");

        let config = create_test_config(config_path.clone(), temp_dir.path().join("backups"));
        config.save(None).unwrap();

        let overrides = CliOverrides {
            port: Some(8080),
            host: Some("127.0.0.1".to_string()),
            timeout: Some(60),
            mock_mode: Some(false),
        };

        let result = AppConfig::load(&config_path, Some(&overrides)).unwrap();

        assert_eq!(result.server.port, 8080);
        assert_eq!(result.server.host, "127.0.0.1");
        assert_eq!(result.server.request_timeout_seconds, 60);
        assert!(!result.hardware.mock_mode);
    }
}
