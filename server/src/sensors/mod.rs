mod ads1115;

use crate::types::Temp;
use serde::{Deserialize, Serialize};
use std::{
    io,
    time::{Duration, SystemTime},
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
pub struct DummySensorDef {
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GpioSensorDef {
    pin: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SensorDef {
    Dummy(DummySensorDef),
    Gpio(GpioSensorDef),
    ADS1115(ads1115::SensorDef),
}
impl SensorDef {
    pub fn to_instance(&self) -> Result<Box<dyn SensorInstance + Send>> {
        match self {
            SensorDef::Dummy(DummySensorDef { file }) => {
                Ok(Box::new(DummySensorInstance::load(file)?))
            }
            SensorDef::Gpio(_) => Ok(Box::new(GpioSensorInstance)),
            SensorDef::ADS1115(data) => Ok(Box::new(ads1115::SensorInstance::new(data)?)),
        }
    }
}

pub trait SensorInstance {
    fn poll(&mut self, time: &SystemTime) -> Result<Temp>;
}

type DummyData = Vec<(u64, f64)>;

struct DummySensorInstance {
    file_data: DummyData,
    start_time: Option<SystemTime>,
}
impl DummySensorInstance {
    fn load(file: &Option<String>) -> Result<DummySensorInstance> {
        match file {
            Some(file) => {
                let data_file = std::fs::File::open(file)?;
                let mut file_data: DummyData = serde_json::from_reader(data_file)?;
                file_data.sort_by_key(|v| v.0);
                Ok(DummySensorInstance {
                    file_data,
                    start_time: None,
                })
            }
            None => Ok(DummySensorInstance {
                file_data: Vec::<(u64, f64)>::from_iter(
                    (0..(15 * 60)).map(|o| (o * 10, 20.0 + (o as f64) * 2.0)),
                ),
                start_time: None,
            }),
        }
    }
}
impl SensorInstance for DummySensorInstance {
    fn poll(&mut self, time: &SystemTime) -> Result<Temp> {
        if self.start_time.is_none() {
            self.start_time = Some(time.clone());
        };
        let start_time = &self.start_time.unwrap();
        // now find the last elem which has a timestamp <= to time
        let next_value = self
            .file_data
            .iter()
            .rfind(|(offset, _)| *start_time + Duration::from_secs(*offset) <= *time);
        match next_value {
            Some((_, temp)) => Ok(*temp),
            None => {
                log::info!("Restarting sample data");
                // we hit the end of the data
                self.start_time = None;
                Err(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    "end of data",
                )))
            }
        }
    }
}
struct GpioSensorInstance;
impl SensorInstance for GpioSensorInstance {
    fn poll(&mut self, _time: &SystemTime) -> Result<Temp> {
        todo!()
    }
}
