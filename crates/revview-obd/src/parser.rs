// Interprets hexadecimal responses, validates their format, and converts data bytes into measurements.

// Parses the serial response for RPMs and returns its numerical value
pub fn parse_rpm_response(response: &str) -> Option<u16> {
    // Remove spaces, newlines, and the '>' prompt
    let clean: String = response
        .split_whitespace()
        .collect::<String>()
        .trim_end_matches('>')
        .to_uppercase();

    let i = clean.find("410C")?;
    let a = u16::from_str_radix(clean.get(i + 4..i + 6)?, 16).ok()?;
    let b = u16::from_str_radix(clean.get(i + 6..i + 8)?, 16).ok()?;

    Some((a * 256 + b) / 4)
}

// Parses the serial response for KPH and returns its numerical value
pub fn parse_speed_response(_response: &str) -> Option<u16> {
    let speed: Option<u16> = None; // Placeholder value

    return speed;
}

// Parses the serial response for coolant temp and returns its numerical value
pub fn parse_coolant_temp_response(_response: &str) -> Option<i16> {
    let coolant_temp: Option<i16> = None; // Placeholder value

    return coolant_temp;
}

// Parses the serial response for intake temp and returns its numerical value
pub fn parse_intake_temp_response(_response: &str) -> Option<i16> {
    let intake_temp: Option<i16> = None; // Placeholder value

    return intake_temp;
}

// Parses the serial response for throttle position and returns its numerical value
pub fn parse_throttle_position_response(_response: &str) -> Option<f32> {
    let throttle_position: Option<f32> = None; // Placeholder value

    return throttle_position;
}
