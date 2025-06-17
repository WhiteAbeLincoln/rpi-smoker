use super::error::{ConfigError, ConfigResult};
use super::models::*;
use super::validation::ConfigValidator;
use chrono::{DateTime, Utc};
use json_patch::Patch;
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

    /// Replace current configuration with another AppConfig
    /// preserves the loaded_from path on the original config
    fn replace(&mut self, mut other: AppConfig) -> ConfigResult<()> {
        other.loaded_from = self.loaded_from.clone();
        other.populate_names();
        other.validate()?;

        self.backup_current()?;
        // replace the current config with the new one
        *self = other;
        self.save(None)?;

        Ok(())
    }

    /// Update partial configuration (for API updates)
    ///
    /// Applies JSON Patch operations (RFC 6902) to update the configuration.
    ///
    /// # Example JSON Patch Operations
    ///
    /// ```json
    /// [
    ///   {
    ///     "op": "replace",
    ///     "path": "/server/port",
    ///     "value": 8080
    ///   },
    ///   {
    ///     "op": "add",
    ///     "path": "/server/cors_origins/-",
    ///     "value": "https://example.com"
    ///   },
    ///   {
    ///     "op": "remove",
    ///     "path": "/server/cors_origins/0"
    ///   }
    /// ]
    /// ```
    ///
    /// The updated configuration is validated before being applied. If validation
    /// fails, the original configuration remains unchanged.
    pub fn update_partial(&mut self, updates: serde_json::Value) -> ConfigResult<()> {
        // Parse the JSON Patch operations
        let patch: Patch =
            serde_json::from_value(updates).map_err(|e| ConfigError::InvalidJson { source: e })?;

        // Convert self to JSON Value for patch operations
        let mut config_value =
            serde_json::to_value(&self).map_err(|e| ConfigError::InvalidJson { source: e })?;

        // Apply the patch operations
        json_patch::patch(&mut config_value, &patch)
            .map_err(|e| ConfigError::PatchFailed { source: e })?;

        // Convert back to AppConfig and validate
        let updated_config: AppConfig = serde_json::from_value(config_value)
            .map_err(|e| ConfigError::InvalidJson { source: e })?;

        self.replace(updated_config)?;

        Ok(())
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
    pub fn restore_from_backup(&mut self, backup_filename: &str) -> ConfigResult<()> {
        // load backup
        let backup_path = self.server.config_backup_dir.join(backup_filename);
        let config = AppConfig::load(&backup_path, None)?;

        self.replace(config)?;

        Ok(())
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
        config.restore_from_backup(&backups[0].filename).unwrap();
        assert_eq!(config.temp_sensors["probe1"].name, "probe1");

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

    #[test]
    fn test_update_partial_json_patch() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.json");
        let mut config = create_test_config(config_path.clone(), temp_dir.path().join("backups"));

        // Create a JSON patch to update the server port
        let patch = serde_json::json!([
            {
                "op": "replace",
                "path": "/server/port",
                "value": 9000
            }
        ]);

        // Apply the patch
        config.update_partial(patch).unwrap();

        // Verify the port was updated
        assert_eq!(config.server.port, 9000);

        // Verify other fields remain unchanged
        assert_eq!(config.server.host, "0.0.0.0");
    }

    #[test]
    fn test_update_partial_add_operation() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.json");
        let mut config = create_test_config(config_path.clone(), temp_dir.path().join("backups"));

        // Create a JSON patch to add a new CORS origin
        let patch = serde_json::json!([
            {
                "op": "add",
                "path": "/server/cors_origins/-",
                "value": "https://example.com"
            }
        ]);

        // Apply the patch
        config.update_partial(patch).unwrap();

        // Verify the new origin was added
        assert!(
            config
                .server
                .cors_origins
                .contains(&"https://example.com".to_string())
        );
    }

    #[test]
    fn test_update_partial_remove_operation() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.json");
        let mut config = create_test_config(config_path.clone(), temp_dir.path().join("backups"));

        // Ensure we have multiple CORS origins first
        config.server.cors_origins = vec!["*".to_string(), "http://localhost:3000".to_string()];

        // Create a JSON patch to remove the first CORS origin
        let patch = serde_json::json!([
            {
                "op": "remove",
                "path": "/server/cors_origins/0"
            }
        ]);

        // Apply the patch
        config.update_partial(patch).unwrap();

        // Verify the origin was removed
        assert_eq!(config.server.cors_origins.len(), 1);
        assert_eq!(config.server.cors_origins[0], "http://localhost:3000");
    }

    #[test]
    fn test_update_partial_invalid_patch() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.json");
        let mut config = create_test_config(config_path.clone(), temp_dir.path().join("backups"));

        // Create an invalid JSON patch (missing required field)
        let invalid_patch = serde_json::json!([
            {
                "op": "replace",
                "path": "/server/port"
                // missing "value" field
            }
        ]);

        // Apply the patch and expect an error
        let result = config.update_partial(invalid_patch);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_partial_validation_failure() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.json");
        let mut config = create_test_config(config_path.clone(), temp_dir.path().join("backups"));

        // Create a JSON patch that would result in invalid configuration
        let patch = serde_json::json!([
            {
                "op": "replace",
                "path": "/server/port",
                "value": 999  // Invalid port (below 1024)
            }
        ]);

        // Apply the patch and expect a validation error
        let result = config.update_partial(patch);
        assert!(result.is_err());

        // Verify original config is unchanged after validation failure
        assert_ne!(config.server.port, 999);
    }

    #[test]
    fn test_update_partial_preserves_loaded_from() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.json");
        let mut config = create_test_config(config_path.clone(), temp_dir.path().join("backups"));

        let original_loaded_from = config.loaded_from.clone();

        // Create a simple patch
        let patch = serde_json::json!([
            {
                "op": "replace",
                "path": "/server/host",
                "value": "0.0.0.0"
            }
        ]);

        // Apply the patch
        config.update_partial(patch).unwrap();

        // Verify loaded_from is preserved
        assert_eq!(config.loaded_from, original_loaded_from);

        // Verify the update was applied
        assert_eq!(config.server.host, "0.0.0.0");
    }
}
