// ADDING A NEW SETTING: checklist
//
// 1. Here: add the field to `AppSettings` and its value in `Default`
//    (plus any enum or MIN/MAX consts it needs).
// 2. main.rs, startup: call `app.set_<name>(...)` with the loaded value.
//    Skipping this means the UI starts at 0 and the next save overwrites it.
// 3. main.rs, `on_settings_changed`: read `ui.get_<name>()`, convert and
//    clamp it, and add it to the `AppSettings` that gets saved.
// 4. dashboard.slint: add an `in-out property` and bind it to `Settings`
//    with `<name> <=> root.<name>;`.
// 5. settings.slint: add an `in-out property` and a control that calls
//    `root.settings-changed()` on change.
// 6. Wherever it's used (e.g. speedometer.slint): add an `in property`,
//    pass it down from `Dashboard`, and use it.
//
// Naming: Slint is kebab-case (`shift-rpm`), Rust is snake_case
// (`get_shift_rpm` / `set_shift_rpm`).

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub speed_unit: SpeedUnit,
    pub temperature_unit: TemperatureUnit,
    pub redline_rpm: u32,
    pub shift_rpm: u32,
    pub gauge_style: u32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            speed_unit: SpeedUnit::default(),
            temperature_unit: TemperatureUnit::default(),
            redline_rpm: 6500,
            shift_rpm: 6000,
            gauge_style: 0,
        }
    }
}
