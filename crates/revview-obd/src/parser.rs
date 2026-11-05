// Interprets hexadecimal responses, validates their format, and converts data bytes into measurements.

// Parses the serial response for RPMs and returns its numerical value
pub fn parse_rpm_response(response: &str) -> Option<u16> {
    let bytes: Vec<&str> = response.split_whitespace().collect();

    let index = bytes.windows(2).position(|window| window == ["41", "0C"])?;

    let a = u16::from_str_radix(bytes[index + 2], 16).ok()?;
    let b = u16::from_str_radix(bytes[index + 3], 16).ok()?;

    Some((a * 256 + b) / 4)
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
