use std::sync::{Arc, RwLock};

use crate::config::AppConfig;

/// Application state shared across handlers and sensor threads
#[derive(Clone)]
pub struct AppState {
    config: Arc<RwLock<AppConfig>>,
}

impl AppState {
    /// Create a new application state with the given configuration
    pub fn new(config: AppConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
        }
    }

    /// Get a read lock on the configuration
    pub fn read_config(&'_ self) -> std::sync::RwLockReadGuard<'_, AppConfig> {
        // we use `unwrap()` here because we expect the lock to succeed
        // the only case it would fail is if the lock is poisoned, which we can't recover from
        self.config.read().unwrap()
    }

    /// Get a write lock on the configuration
    pub fn write_config(&'_ self) -> std::sync::RwLockWriteGuard<'_, AppConfig> {
        // we use `unwrap()` here because we expect the lock to succeed
        // the only case it would fail is if the lock is poisoned, which we can't recover from
        self.config.write().unwrap()
    }
}
