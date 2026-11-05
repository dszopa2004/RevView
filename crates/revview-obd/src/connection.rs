// Manages the serial connection, including opening the port, sending requests, and receiving responses.

use serialport::{self, Error};
use std::io::{self, Write};
use std::time::Duration;

static port_name: &str = "/dev/ttyUSB0";
static baud_rate: u32 = 9600;

// Opens serial connection to /dev/ttyUSB0
pub fn open_connection() {
    // Open the port
    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open();

    // Check if port connected successfully
    match port {
        Ok(_port) => {
            println!("Successfully opened port: {}", port_name);
        }
        Err(e) => {
            eprintln!("Failed to open port {}. Error: {}", port_name, e);
            std::process::exit(1);
        }
    }
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
pub fn read_response() {}

// Closes the connection (usually automatic when dropped)
pub fn close_connection() {}
