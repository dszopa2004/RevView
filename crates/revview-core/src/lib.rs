// Declares public modules and exposes commonly used types to other crates.

pub mod calculations;
pub mod parameters;
pub mod settings;
pub mod storage;
pub mod telemetry;

pub use settings::{AppSettings, SpeedUnit, TemperatureUnit};
pub use storage::SettingsStorage;
pub use telemetry::VehicleTelemetry;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
