use std::{
    collections::HashMap,
    error::Error,
    time::{Duration, Instant},
};

use revview_core::vehicle::{ExtraPid, VehicleProfile};

use crate::{
    connection::{read_response, send_command},
    parser,
};

/// Reads one extra PID. Always restores the default header afterward, or
/// the generic polling would stop getting answers.
pub fn read_extra(
    port: &mut dyn serialport::SerialPort,
    pid: &ExtraPid,
) -> Result<Option<f32>, Box<dyn Error>> {
    if let Some(header) = pid.header {
        send_command(port, &format!("ATSH{header}\r"))?;
        read_response(port)?;
    }

    let reply = (|| -> Result<String, Box<dyn Error>> {
        send_command(port, pid.command)?;
        Ok(read_response(port)?)
    })();

    if pid.header.is_some() {
        let _ = send_command(port, "ATSH7DF\r"); // default 11-bit CAN header
        let _ = read_response(port);
    }

    let reply = reply?;

    // Response mode = request mode + 0x40 (e.g. "2101" -> 0x61)
    let request_mode = u8::from_str_radix(pid.command.get(..2).ok_or("short command")?, 16)?;
    let data = parser::parse_extra_response(&reply, request_mode.wrapping_add(0x40));

    Ok(data.and_then(|d| (pid.decode)(&d)))
}

/// Round-robins through a profile's extra PIDs at a fixed interval.
pub struct ExtrasPoller {
    next: usize,
    last: Option<Instant>,
    interval: Duration,
}

impl ExtrasPoller {
    pub fn new(interval: Duration) -> Self {
        Self {
            next: 0,
            last: None,
            interval,
        }
    }

    pub fn due(&self, now: Instant) -> bool {
        self.last
            .is_none_or(|t| now.duration_since(t) >= self.interval)
    }

    /// Caller must check `profile.extra_pids` is non-empty first.
    pub fn poll_next(
        &mut self,
        port: &mut dyn serialport::SerialPort,
        profile: &VehicleProfile,
        values: &mut HashMap<&'static str, f32>,
    ) {
        self.last = Some(Instant::now());
        let pid = &profile.extra_pids[self.next % profile.extra_pids.len()];
        self.next = self.next.wrapping_add(1);

        match read_extra(port, pid) {
            Ok(Some(v)) => {
                values.insert(pid.id, v);
            }
            Ok(None) => {
                values.remove(pid.id);
            }
            Err(e) => eprintln!("Extra PID {} failed: {}", pid.id, e),
        }
    }
}
