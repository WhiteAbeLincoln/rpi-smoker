use ads1x1x::ic::{Ads1115, Resolution16Bit};
use ads1x1x::interface::I2cInterface;
use ads1x1x::mode::OneShot;
use ads1x1x::{Ads1x1x, ChannelSelection, DynamicOneShot, FullScaleRange, SlaveAddr};
use evalexpr::{ContextWithMutableVariables, HashMapContext};
use nb::block;
use rppal::i2c::I2c;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::sensors;
use crate::types::{ParseableExpr, Temp};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub enum ADS1115Gain {
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
impl ADS1115Gain {
    fn to_volts(self) -> f64 {
        match self {
            ADS1115Gain::TwoThirds => 6.144,
            ADS1115Gain::One => 4.096,
            ADS1115Gain::Two => 2.048,
            ADS1115Gain::Four => 1.024,
            ADS1115Gain::Eight => 0.512,
            ADS1115Gain::Sixteen => 0.256,
        }
    }
}
impl Into<FullScaleRange> for ADS1115Gain {
    fn into(self) -> FullScaleRange {
        match self {
            ADS1115Gain::TwoThirds => FullScaleRange::Within6_144V,
            ADS1115Gain::One => FullScaleRange::Within4_096V,
            ADS1115Gain::Two => FullScaleRange::Within2_048V,
            ADS1115Gain::Four => FullScaleRange::Within1_024V,
            ADS1115Gain::Eight => FullScaleRange::Within0_512V,
            ADS1115Gain::Sixteen => FullScaleRange::Within0_256V,
        }
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub enum Channel {
    /// Measure single-ended signal on input channel 0
    #[default]
    SingleA0,
    /// Measure single-ended signal on input channel 1
    SingleA1,
    /// Measure single-ended signal on input channel 2
    SingleA2,
    /// Measure single-ended signal on input channel 3
    SingleA3,
    /// Measure signal on input channel 0 differentially to signal on input channel 1
    DifferentialA0A1,
    /// Measure signal on input channel 0 differentially to signal on input channel 3
    DifferentialA0A3,
    /// Measure signal on input channel 1 differentially to signal on input channel 3
    DifferentialA1A3,
    /// Measure signal on input channel 2 differentially to signal on input channel 3
    DifferentialA2A3,
}
impl Into<ChannelSelection> for Channel {
    fn into(self) -> ChannelSelection {
        match self {
            Channel::SingleA0 => ChannelSelection::SingleA0,
            Channel::SingleA1 => ChannelSelection::SingleA1,
            Channel::SingleA2 => ChannelSelection::SingleA2,
            Channel::SingleA3 => ChannelSelection::SingleA3,
            Channel::DifferentialA0A1 => ChannelSelection::DifferentialA0A1,
            Channel::DifferentialA0A3 => ChannelSelection::DifferentialA0A3,
            Channel::DifferentialA1A3 => ChannelSelection::DifferentialA1A3,
            Channel::DifferentialA2A3 => ChannelSelection::DifferentialA2A3,
        }
    }
}
impl Channel {
    fn to_bits(&self) -> u8 {
        match self {
            Channel::SingleA0 => 15,
            Channel::SingleA1 => 15,
            Channel::SingleA2 => 15,
            Channel::SingleA3 => 15,
            Channel::DifferentialA0A1 => 16,
            Channel::DifferentialA0A3 => 16,
            Channel::DifferentialA1A3 => 16,
            Channel::DifferentialA2A3 => 16,
        }
    }
    fn to_resolution(&self) -> u32 {
        (2 as u32) << ((self.to_bits() - 1) as u32)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SensorDef {
    /// The configured gain. The default is 2, which results in a range of ±2.048V
    #[serde(default = "Default::default")]
    gain: ADS1115Gain,
    /// The channel that we read from
    #[serde(default = "Default::default")]
    channel: Channel,
    #[serde(skip_serializing_if = "Option::is_none")]
    bus: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    v2temp: Option<ParseableExpr>,
}

fn handle_ads_err<'a, E: std::error::Error + 'a>(
    e: ads1x1x::Error<E>,
) -> Box<dyn std::error::Error + 'a> {
    match e {
        ads1x1x::Error::InvalidInputData => panic!("sent invalid input to adc"),
        ads1x1x::Error::I2C(err) => {
            return Box::new(err);
        }
    }
}

pub struct SensorInstance {
    channel: Channel,
    adc: ads1x1x::Ads1x1x<I2cInterface<I2c>, Ads1115, Resolution16Bit, OneShot>,
    v2temp: Option<evalexpr::Node>,
    gain: ADS1115Gain,
    // ref_voltage: f64,
}
impl SensorInstance {
    pub fn new(data: &SensorDef) -> sensors::Result<SensorInstance> {
        let handle = match data.bus {
            Some(bus) => I2c::with_bus(bus),
            None => I2c::new(),
        }?;
        // if let Some(addr) = data.address {
        //     handle.set_slave_address(addr)?;
        // };
        let address = SlaveAddr::default();
        let mut adc = Ads1x1x::new_ads1115(handle, address);
        adc.set_full_scale_range(data.gain.into())
            .map_err(handle_ads_err)?;

        Ok(SensorInstance {
            gain: data.gain,
            adc,
            channel: data.channel, //data.channel,
            v2temp: match &data.v2temp {
                Some(f) => Some(f.data.clone()),
                None => None,
            },
        })
    }
}
impl crate::sensors::SensorInstance for SensorInstance {
    fn poll(&mut self, _time: &SystemTime) -> sensors::Result<Temp> {
        let value = block!(self.adc.read(self.channel.into()));
        match value {
            Ok(val) => {
                // we must convert the raw value to a voltage
                // the relation is RawValue / Resolution = VIN / FSR
                // see: https://e2e.ti.com/support/data-converters-group/data-converters/f/data-converters-forum/821646/ads1115-q1-voltage-reference-value
                // the ADS is 15 bits in single mode (16 in differential), which gives a resolution of 2^15 or 2^16
                // to get VIN (our input voltage) given the raw value, resolution, and gain as volts (FSR)
                // we use the formula VIN = (RawValue / Resolution) * FSR
                // https://arduino.stackexchange.com/a/69317
                let v = (val as f64 / self.channel.to_resolution() as f64) * self.gain.to_volts();

                let temp = match &self.v2temp {
                    Some(expr) => {
                        let mut ctx = HashMapContext::new();
                        // only fails if the types are different between an existing and new value
                        // we don't expect that since we're inserting into an empty context
                        ctx.set_value("v".to_string(), evalexpr::Value::Float(v))
                            .expect("ctx should be empty");
                        ctx.set_value("raw".to_string(), evalexpr::Value::Float(val.into()))
                            .expect("ctx should be empty");
                        ctx.set_value(
                            "FSR".to_string(),
                            evalexpr::Value::Float(self.gain.to_volts()),
                        )
                        .expect("ctx should be empty");
                        ctx.set_value(
                            "Resolution".to_string(),
                            evalexpr::Value::Float(self.channel.to_resolution().into()),
                        )
                        .expect("ctx should be empty");
                        expr.eval_float_with_context_mut(&mut ctx)?
                    }
                    None => v.into(),
                };

                log::debug!("got {}: {}V, {}F from adc {:?}", val, v, temp, self.channel);

                Ok(temp)
            }
            Err(e) => Err(handle_ads_err(e)),
        }
    }
}
