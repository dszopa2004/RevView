// Interprets hexadecimal responses, validates their format, and converts data bytes into measurements.

// Cleans the response and extracts the data bytes for a specified OBD-II PID.
fn parse_obd_response(response: &str, pid: &str) -> Option<Vec<u8>> {
    let clean: String = response
        .split_whitespace()
        .collect::<String>()
        .trim_end_matches('>')
        .to_uppercase();

    let header = format!("41{}", pid);
    let i = clean.find(&header)?;

    let data = clean.get(i + header.len()..)?;

    // Convert the remaining hexadecimal pairs into bytes.
    // Stop if the next character is not valid hexadecimal data.
    let mut bytes = Vec::new();

    for pair in data.as_bytes().chunks_exact(2) {
        let pair = std::str::from_utf8(pair).ok()?;
        bytes.push(u8::from_str_radix(pair, 16).ok()?);
    }

    Some(bytes)
}

// Parses the serial response for RPMs and returns its numerical value.
pub fn parse_rpm_response(response: &str) -> Option<u16> {
    let bytes = parse_obd_response(response, "0C")?;

    let a = *bytes.first()? as u16;
    let b = *bytes.get(1)? as u16;

    Some((a * 256 + b) / 4)
}

// Parses the serial response for KPH and returns its numerical value.
pub fn parse_speed_response(response: &str) -> Option<u16> {
    let bytes = parse_obd_response(response, "0D")?;

    Some(*bytes.first()? as u16)
}

// Parses the serial response for coolant temperature in Celsius.
pub fn parse_coolant_temp_response(response: &str) -> Option<i16> {
    let bytes = parse_obd_response(response, "05")?;

    Some(*bytes.first()? as i16 - 40)
}

// Parses the serial response for intake temperature in Celsius.
pub fn parse_intake_temp_response(response: &str) -> Option<i16> {
    let bytes = parse_obd_response(response, "0F")?;

    Some(*bytes.first()? as i16 - 40)
}

// Parses the serial response for throttle position as a percentage.
pub fn parse_throttle_position_response(response: &str) -> Option<f32> {
    let bytes = parse_obd_response(response, "11")?;

    Some((*bytes.first()? as f32 / 255.0) * 100.0)
}

pub fn parse_stored_codes_response(response: &str) {}
pub fn parse_pending_codes_response(response: &str) {}
pub fn parse_permanent_codes_response(response: &str) {}
