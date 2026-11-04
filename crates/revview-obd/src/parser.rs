// Parses the serial response for RPMs and returns its numerical value
pub fn parse_rpm_response(response: &str) -> u16 {
    // todo: extract A and B bits from response
    let a: u8 = u8::from_str_radix("1F", 16).unwrap();
    let b: u8 = u8::from_str_radix("40", 16).unwrap();

    let rpm: u16 = ((256 * a as u16) + b as u16) / 4;

    return rpm as u16;
}

// Parses the serial response for KPH and returns its numerical value
pub fn parse_speed_response(response: &str) -> u16 {
    let speed: u16 = 1000; // Placeholder value

    return speed;
}

// Parses the serial response for coolant temp and returns its numerical value
pub fn parse_coolant_temp_response(response: &str) -> i16 {
    let coolant_temp: i16 = 1000; // Placeholder value

    return coolant_temp;
}

// Parses the serial response for intake temp and returns its numerical value
pub fn parse_intake_temp_response(response: &str) -> i16 {
    let intake_temp: i16 = 1000; // Placeholder value

    return intake_temp;
}

// Parses the serial response for throttle position and returns its numerical value
pub fn parse_throttle_position_response(response: &str) -> f32 {
    let throttle_position: f32 = 1000.0; // Placeholder value

    return throttle_position;
}
