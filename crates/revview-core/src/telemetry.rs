// Defines VehicleTelemetry, the structure holding current vehicle measurements.

#[derive(Debug, Clone, Default)]
pub struct VehicleTelemetry {
    pub rpm: Option<u16>,
    pub speed_kph: Option<u16>,
    pub coolant_temp_c: Option<i16>,
    pub intake_temp_c: Option<i16>,
    pub throttle_position: Option<f32>,
}
