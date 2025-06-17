pub mod api;
pub mod config;
pub mod error;
pub mod state;

pub use error::{AppError, AppResult};

/// Re-export commonly used types and traits
pub mod prelude {
    pub use crate::{AppError, AppResult};
    pub use axum::{
        Router,
        extract::{Path, Query, State},
        http::StatusCode,
        response::{IntoResponse, Json, Response},
        routing::{delete, get, post, put},
    };
    pub use serde::{Deserialize, Serialize};
    pub use thiserror::Error;
    pub use tracing::{debug, error, info, warn};
}

#[cfg(all(feature = "rpi-hardware", target_os = "linux"))]
pub mod hardware {
    //! Hardware abstraction layer for Raspberry Pi GPIO and sensors
    pub use rppal::gpio::{Gpio, InputPin, OutputPin};

    pub struct HardwareManager {
        gpio: Gpio,
    }

    impl HardwareManager {
        pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
            let gpio = Gpio::new()?;
            Ok(Self { gpio })
        }

        pub fn get_pin(&self, pin: u8) -> Result<OutputPin, Box<dyn std::error::Error>> {
            Ok(self.gpio.get(pin)?.into_output())
        }
    }
}
