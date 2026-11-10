// Performs calculations using vehicle measurements, such as unit conversions and derived values.
pub enum SpeedUnit {
    Kph,
    Mph,
}

pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

// Convert kilometers per hour to miles per hour
pub fn kph_to_mph(kph: u16) -> f32 {
    kph as f32 / 1.609344
}

// Converts celsius to fahrenheit
pub fn celsius_to_fahrenheit(celsius: i16) -> f32 {
    (celsius as f32 * 9.0 / 5.0) + 32.0
}

pub fn convert_speed(speed: u16, unit: SpeedUnit) -> f32 {
    match unit {
        SpeedUnit::Kph => speed as f32,
        SpeedUnit::Mph => kph_to_mph(speed),
    }
}

pub fn convert_temp(temp: i16, unit: TemperatureUnit) -> f32 {
    match unit {
        TemperatureUnit::Celsius => temp as f32,
        TemperatureUnit::Fahrenheit => celsius_to_fahrenheit(temp),
    }
}
