// Application entry point. Initializes the UI and starts the application.

use std::{thread, time::Duration};

use revview_core::VehicleTelemetry;
use revview_obd::{
    commands,
    connection::{initialize_adapter, open_connection, read_response, send_command},
    parser,
};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = Dashboard::new()?;

    // Create a weak handle to safely pass across thread boundaries
    let app_weak = app.as_weak();

    // Spawn a background thread to handle the blocking OBD loop
    thread::spawn(move || {
        let mut telemetry = VehicleTelemetry::default();

        match open_connection() {
            Ok(mut port) => {
                let app_copy = app_weak.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = app_copy.upgrade() {
                        ui.set_obd_connected(true); // Set if OBD-II is connected successfully
                    }
                });
                println!("Successfully opened the port!");

                if let Err(e) = initialize_adapter(&mut port) {
                    let app_copy = app_weak.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(ui) = app_copy.upgrade() {
                            ui.set_obd_initialized(false); // Set if OBD-II is not initialized successfully
                        }
                    });
                    eprintln!("Failed to initialize adapter: {}", e);
                    return;
                }

                let app_copy = app_weak.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = app_copy.upgrade() {
                        ui.set_obd_initialized(true); // Set if OBD-II is initialized successfully
                    }
                });

                println!("Adapter initialized successfully!");

                // Infinite loop to continuously update telemetry while the app runs
                loop {
                    if let Err(e) = send_command(&mut port, commands::RPM_COMMAND) {
                        eprintln!("Failed to send command: {}", e);
                        thread::sleep(Duration::from_millis(500));
                        continue;
                    }

                    match read_response(&mut port) {
                        Ok(response) => {
                            println!("Success! Received bytes: {:?}", response);
                            telemetry.rpm = parser::parse_rpm_response(&response);
                            println!("RPMs: {:?}", telemetry.rpm);

                            // Clone weak handle and upgrade it on the UI thread to push updates
                            let app_copy = app_weak.clone();
                            let current_rpm = telemetry.rpm.unwrap_or(0) as f32;

                            let _ = slint::invoke_from_event_loop(move || {
                                if let Some(ui) = app_copy.upgrade() {
                                    // Assigning the RPM data to the Slint property
                                    ui.set_rpm(current_rpm as f32);
                                }
                            });
                        }
                        Err(e) => {
                            eprintln!("Error reading from port: {}", e);
                        }
                    }

                    // Throttle the OBD requests to avoid flooding the bus
                    thread::sleep(Duration::from_millis(100));
                }
            }
            Err(e) => {
                let app_copy = app_weak.clone();

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = app_copy.upgrade() {
                        ui.set_obd_connected(false); // Set if OBD-II is connected successfully
                    }
                });
                eprintln!("Failed to open port: {}", e);
            }
        }
    });

    // Start the event loop on the main thread (blocks here until the window closes)
    app.run()
}
