use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeedUnit {
    #[default]
    Kph,
    Mph,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemperatureUnit {
    #[default]
    Celsius,
    Fahrenheit,
}

pub const REDLINE_MIN: u32 = 3000;
pub const REDLINE_MAX: u32 = 10000;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AppSettings {
    pub speed_unit: SpeedUnit,
    pub temperature_unit: TemperatureUnit,
    pub redline_rpm: u32,
}
