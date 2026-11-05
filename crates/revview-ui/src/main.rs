// Application entry point. Initializes the UI and starts the application.

use revview_obd::connection::open_connection;

slint::include_modules!();

fn main() {
    // -> Result<(), slint::PlatformError>
    //let app = Dashboard::new()?;
    //app.run()
    match open_connection() {
        Ok(mut port) => {
            println!("Successfully opened the port!");
            // Read/write using `port`
        }
        Err(e) => eprintln!("Failed to open port: {}", e),
    }
}
