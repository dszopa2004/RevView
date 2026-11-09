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
                update_ui(&app_weak, move |ui| {
                    ui.set_obd_connected(true);
                });
                println!("Successfully opened the port!");

                if let Err(e) = initialize_adapter(&mut port) {
                    update_ui(&app_weak, move |ui| {
                        ui.set_obd_initialized(false);
                    });
                    eprintln!("Failed to initialize adapter: {}", e);
                    let error_msg = format!("Failed to initialize adapter: {}", e);

                    update_ui(&app_weak, move |ui| {
                        ui.set_obd_initialization_error(error_msg.into());
                    });
                    return;
                }

                update_ui(&app_weak, move |ui| {
                    ui.set_obd_initialized(true);
                });

                println!("Adapter initialized successfully!");

                // Infinite loop to continuously update telemetry while the app runs
                loop {
                    // RPM
                    match request_telemetry(&mut *port, commands::RPM_COMMAND) {
                        Ok(response) => {
                            telemetry.rpm = parser::parse_rpm_response(&response);
                        }
                        Err(e) => eprintln!("RPM request failed: {}", e),
                    }

                    // Speed
                    match request_telemetry(&mut *port, commands::SPEED_COMMAND) {
                        Ok(response) => {
                            telemetry.speed_kph = parser::parse_speed_response(&response);
                        }
                        Err(e) => eprintln!("Speed request failed: {}", e),
                    }

                    // Coolant Temp
                    match request_telemetry(&mut *port, commands::COOLANT_TEMP_COMMAND) {
                        Ok(response) => {
                            telemetry.coolant_temp_c =
                                parser::parse_coolant_temp_response(&response);
                        }
                        Err(e) => eprintln!("Coolant temp request failed: {}", e),
                    }

                    // Intake Temp
                    match request_telemetry(&mut *port, commands::INTAKE_TEMP_COMMAND) {
                        Ok(response) => {
                            telemetry.intake_temp_c = parser::parse_intake_temp_response(&response);
                        }
                        Err(e) => eprintln!("Intake temp request failed: {}", e),
                    }

                    // Throttle Position
                    match request_telemetry(&mut *port, commands::THROTTLE_POSITION_COMMAND) {
                        Ok(response) => {
                            telemetry.throttle_position =
                                parser::parse_throttle_position_response(&response);
                        }
                        Err(e) => eprintln!("Throttle position request failed: {}", e),
                    }

                    let rpm = telemetry.rpm.unwrap_or(0) as f32;
                    let speed = telemetry.speed_kph.unwrap_or(0) as f32;
                    let coolant_temp = telemetry.coolant_temp_c.unwrap_or(0) as f32;
                    let intake_temp = telemetry.intake_temp_c.unwrap_or(0) as f32;
                    let throttle_position = telemetry.throttle_position.unwrap_or(0.0) as f32;

                    update_ui(&app_weak, move |ui| {
                        ui.set_rpm(rpm);
                        ui.set_speed(speed);
                        ui.set_coolant_temp(coolant_temp);
                        ui.set_intake_temp(intake_temp);
                        ui.set_throttle_position(throttle_position);
                    });

                    thread::sleep(Duration::from_millis(100));
                }
            }
            Err(e) => {
                update_ui(&app_weak, move |ui| {
                    ui.set_obd_connected(false);
                });
                eprintln!("Failed to open port: {}", e);
            }
        }
    });

    // Start the event loop on the main thread (blocks here until the window closes)
    app.run()
}

fn update_ui(app_weak: &slint::Weak<Dashboard>, update: impl FnOnce(Dashboard) + Send + 'static) {
    let app_weak = app_weak.clone();

    let _ = slint::invoke_from_event_loop(move || {
        if let Some(ui) = app_weak.upgrade() {
            update(ui);
        }
    });
}

fn request_telemetry(
    port: &mut dyn serialport::SerialPort,
    command: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    send_command(port, command)?;
    let response = read_response(port)?;

    Ok(response)
}
