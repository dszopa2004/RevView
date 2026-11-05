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
        .timeout(Duration::from_millis(10))
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
pub fn read_response(port: &mut impl Read) -> io::Result<[u8; 4]> {
    // Allocate the storage first
    let mut buffer = [0u8; 4];

    // Pass a mutable reference.
    // If successful, `_` is just `()`, and `buffer` now holds your data.
    port.read_exact(&mut buffer)?;

    // Return your populated buffer
    Ok(buffer)
}

// Closes the connection (usually automatic when dropped)
pub fn close_connection() {}
