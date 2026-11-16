// Interprets hexadecimal responses, validates their format, and converts data bytes into measurements.

use revview_core::diagnostics::DiagnosticCode;

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

pub fn parse_stored_codes_response(response: &str) -> Option<Vec<DiagnosticCode>> {
    parse_codes(response, 0x43)
}
pub fn parse_pending_codes_response(response: &str) -> Option<Vec<DiagnosticCode>> {
    parse_codes(response, 0x47)
}
pub fn parse_permanent_codes_response(response: &str) -> Option<Vec<DiagnosticCode>> {
    parse_codes(response, 0x4A)
}

fn decode_dtc(a: u8, b: u8) -> DiagnosticCode {
    let system = ['P', 'C', 'B', 'U'][(a >> 6) as usize];
    let digit1 = (a >> 4) & 0b11;
    let digit2 = a & 0x0F;
    DiagnosticCode(format!("{system}{digit1}{digit2:X}{b:02X}"))
}

/// Turns raw adapter text into one byte-vector per ECU message,
/// joining ELM327 multi-frame output ("0: ...", "1: ...").
fn split_messages(response: &str) -> Vec<Vec<u8>> {
    let mut messages: Vec<Vec<u8>> = Vec::new();

    for line in response.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let is_frame = line.as_bytes().get(1) == Some(&b':');
        let body = if is_frame { &line[2..] } else { line };

        // Skips "SEARCHING...", "NO DATA", and the 3-digit length line.
        let tokens: Vec<&str> = body.split_whitespace().collect();
        if tokens.is_empty() || tokens.iter().any(|t| t.len() != 2) {
            continue;
        }
        let Some(bytes) = tokens
            .iter()
            .map(|t| u8::from_str_radix(t, 16).ok())
            .collect::<Option<Vec<u8>>>()
        else {
            continue;
        };

        let continues_previous = is_frame && !line.starts_with("0:");
        match (continues_previous, messages.last_mut()) {
            (true, Some(last)) => last.extend(bytes),
            _ => messages.push(bytes),
        }
    }
    messages
}

fn parse_codes(response: &str, header: u8) -> Option<Vec<DiagnosticCode>> {
    let mut codes = Vec::new();
    let mut saw_reply = false;

    for message in split_messages(response) {
        let Some((&h, data)) = message.split_first() else {
            continue;
        };
        if h != header {
            continue;
        }
        saw_reply = true;

        // CAN inserts a code-count byte after the header, so an odd
        // number of data bytes means it's present.
        let data = if data.len() % 2 == 1 {
            &data[1..]
        } else {
            data
        };

        for pair in data.chunks_exact(2) {
            if pair == [0, 0] {
                continue; // padding
            }
            let code = decode_dtc(pair[0], pair[1]);
            if !codes.contains(&code) {
                codes.push(code);
            }
        }
    }

    saw_reply.then_some(codes)
}
