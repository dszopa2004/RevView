// Defines application settings and their default values.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeedUnit {
    Kph,
    Mph,
}

impl Default for SpeedUnit {
    fn default() -> Self {
        Self::Kph
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub speed_unit: SpeedUnit,
    pub temperature_unit: TemperatureUnit,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            speed_unit: SpeedUnit::Kph,
            temperature_unit: TemperatureUnit::Celsius,
        }
    }
}
