// Application entry point. Initializes the UI and starts the application.

use revview_obd::{
    commands,
    connection::{open_connection, read_response, send_command},
};

slint::include_modules!();

fn main() {
    // -> Result<(), slint::PlatformError>
    //let app = Dashboard::new()?;
    //app.run()

    // Open connection
    match open_connection() {
        Ok(mut port) => {
            println!("Successfully opened the port!");

            // Handle the write result
            if let Err(e) = send_command(&mut port, commands::RPM_COMMAND) {
                eprintln!("Failed to send command: {}", e);
                return; // Exit early since we can't read a response to a failed command
            }

            // Read the response safely
            match read_response(&mut port) {
                Ok(response) => {
                    println!("Success! Received bytes: {:?}", response);

                    // TODO: Pass this data to Slint later
                }
                Err(e) => {
                    eprintln!("Error reading from port: {}", e);
                    return;
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open port: {}", e);
        }
    }
}
