// Manages the serial connection, including opening the port, sending requests, and receiving responses.

use serialport::{Result, SerialPort};
use std::io::{self, ErrorKind, Read, Write};
use std::time::Duration;

static PORT_NAME: &str = "/dev/ttyUSB0";
static BAUD_RATE: u32 = 9600;

// Opens and returns serial connection to /dev/ttyUSB0
pub fn open_connection() -> Result<Box<dyn SerialPort>> {
    // Open the port
    let port = serialport::new(PORT_NAME, BAUD_RATE)
        .timeout(Duration::from_millis(500))
        .open()?;

    Ok(port)
}

// Sends a command to the open connection
pub fn send_command(port: &mut impl Write, command: &str) -> io::Result<()> {
    if let Err(e) = port.write_all(command.as_bytes()) {
        eprintln!("Failed to write to port: {}", e);
    }

    port.flush()?;

    Ok(())
}

// Reads the adapter's response
// Reads the adapter's response until the '>' prompt is found
pub fn read_response(port: &mut impl Read) -> io::Result<String> {
    let mut full_buffer = Vec::new();
    let mut single_byte = [0u8; 1]; // Read 1 byte at a time to check for the delimiter

    loop {
        match port.read_exact(&mut single_byte) {
            Ok(()) => {
                let byte = single_byte[0];
                full_buffer.push(byte);

                // Stop reading immediately once the prompt character is found
                if byte == b'>' {
                    break;
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                // If a timeout happens before we see '>', return what we have so far
                if full_buffer.is_empty() {
                    return Err(io::Error::new(
                        io::ErrorKind::TimedOut,
                        "Read timed out with no data",
                    ));
                }
                break;
            }
            Err(e) => return Err(e),
        }
    }

    // Convert the raw bytes into a valid UTF-8 String
    String::from_utf8(full_buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

// Closes the connection (usually automatic when dropped)
pub fn close_connection() {}
