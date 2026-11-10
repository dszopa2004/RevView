// Manages the serial connection, including opening the port, sending requests, and receiving responses.

use serialport::{Result, SerialPort};
use std::io::{self, Read, Write};
use std::thread;
use std::time::Duration;

const NORMAL_TIMEOUT: Duration = Duration::from_millis(500);
const WARMUP_TIMEOUT: Duration = Duration::from_secs(10);

static PORT_NAME: &str = "/dev/ttyUSB0";
static BAUD_RATE: u32 = 38400; // Try 38400 if ATZ returns nothing readable

// Forces the adapter to finish protocol detection before real polling starts
fn warm_up_protocol(port: &mut Box<dyn SerialPort>) -> io::Result<()> {
    port.set_timeout(WARMUP_TIMEOUT)?;
    port.clear(serialport::ClearBuffer::Input)?;
    send_command(port, "0100\r")?;

    let result = read_response(port);

    // Always restore the short timeout, even if the read failed
    port.set_timeout(NORMAL_TIMEOUT)?;

    let response = result?;
    println!("0100 -> {}", response.trim());

    // Strip whitespace so this works with spaces on or off
    let compact: String = response.split_whitespace().collect();
    if !compact.contains("4100") {
        return Err(io::Error::new(
            io::ErrorKind::NotConnected,
            format!(
                "Car did not respond (is the ignition on?): {}",
                response.trim()
            ),
        ));
    }

    Ok(())
}

// Initializes the ELM327 adapter
pub fn initialize_adapter(port: &mut Box<dyn SerialPort>) -> io::Result<()> {
    let commands = [
        ("ATZ\r", 1000),
        ("ATE0\r", 100),
        ("ATL0\r", 100),
        ("ATS0\r", 100),
        ("ATH0\r", 100),
        ("ATAT1\r", 100),
        ("ATSP0\r", 100),
    ];

    for (command, delay_ms) in commands {
        port.clear(serialport::ClearBuffer::Input)?;
        send_command(port, command)?;
        thread::sleep(Duration::from_millis(delay_ms));

        let response = read_response(port)?; // should read until '>'
        println!("{} -> {}", command.trim(), response.trim());

        // ATZ returns a banner rather than OK
        if command != "ATZ\r" && !response.contains("OK") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Unexpected response to {}: {}",
                    command.trim(),
                    response.trim()
                ),
            ));
        }
    }

    warm_up_protocol(port)?;
    Ok(())
}

// Opens and returns serial connection to /dev/ttyUSB0
pub fn open_connection() -> Result<Box<dyn SerialPort>> {
    // Open the port
    let port = serialport::new(PORT_NAME, BAUD_RATE)
        .timeout(Duration::from_millis(WARMUP_TIMEOUT.as_millis() as u64))
        .open()?;

    Ok(port)
}

// Sends a command to the open connection
pub fn send_command<T: Write + ?Sized>(port: &mut T, command: &str) -> io::Result<()> {
    port.write_all(command.as_bytes())?;
    port.flush()?;

    Ok(())
}

// Reads the adapter's response
// Reads the adapter's response until the '>' prompt is found
pub fn read_response<T: Read + ?Sized>(port: &mut T) -> io::Result<String> {
    let mut full_buffer = Vec::new();
    let mut single_byte = [0u8; 1]; // Read 1 byte at a time to check for the delimiter

    loop {
        match port.read_exact(&mut single_byte) {
            Ok(()) => {
                let byte = single_byte[0];
                full_buffer.push(byte);

                // Stop reading once the prompt character is found
                if byte == b'>' {
                    break;
                }
            }
            Err(e) => return Err(e),
        }
    }

    // Convert the raw bytes into a valid UTF-8 String
    String::from_utf8(full_buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

// Closes the connection (usually automatic when dropped)
pub fn close_connection() {}
