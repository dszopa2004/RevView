// Application entry point. Initializes the UI and starts the application.

use std::{
    thread,
    time::{Duration, Instant},
};

use revview_core::VehicleTelemetry;
use revview_obd::{
    connection::{initialize_adapter, open_connection, read_response, send_command},
    parser,
    scheduler::{PollingScheduler, TelemetryKind},
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

                let mut scheduler = PollingScheduler::new();

                loop {
                    let now = Instant::now();

                    if let Some(kind) = scheduler.next_due(now) {
                        // Send the selected OBD-II request.
                        let result = request_telemetry(&mut *port, kind.command());

                        // Mark the request as completed, even if it failed.
                        scheduler.complete(kind, Instant::now());

                        match result {
                            Ok(response) => {
                                process_telemetry(kind, &response, &mut telemetry);
                            }
                            Err(e) => {
                                eprintln!("{:?} request failed: {}", kind, e);
                            }
                        }

                        // Publish the latest collected telemetry.
                        let rpm = telemetry.rpm.unwrap_or(0) as f32;
                        let speed = telemetry.speed_kph.unwrap_or(0) as f32;
                        let coolant_temp = telemetry.coolant_temp_c.unwrap_or(0) as f32;
                        let intake_temp = telemetry.intake_temp_c.unwrap_or(0) as f32;
                        let throttle_position = telemetry.throttle_position.unwrap_or(0.0);

                        update_ui(&app_weak, move |ui| {
                            ui.set_rpm(rpm);
                            ui.set_speed(speed);
                            ui.set_coolant_temp(coolant_temp);
                            ui.set_intake_temp(intake_temp);
                            ui.set_throttle_position(throttle_position);
                        });
                    } else {
                        // Nothing is due yet. Sleep until the next request can be sent.
                        let wait = scheduler.time_until_next_request(Instant::now());

                        thread::sleep(wait.max(Duration::from_millis(1)));
                    }
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

fn process_telemetry(
    kind: revview_obd::scheduler::TelemetryKind,
    response: &str,
    telemetry: &mut VehicleTelemetry,
) {
    use revview_obd::scheduler::TelemetryKind;

    match kind {
        TelemetryKind::Rpm => {
            telemetry.rpm = parser::parse_rpm_response(response);
        }
        TelemetryKind::Speed => {
            telemetry.speed_kph = parser::parse_speed_response(response);
        }
        TelemetryKind::CoolantTemp => {
            telemetry.coolant_temp_c = parser::parse_coolant_temp_response(response);
        }
        TelemetryKind::IntakeTemp => {
            telemetry.intake_temp_c = parser::parse_intake_temp_response(response);
        }
        TelemetryKind::ThrottlePosition => {
            telemetry.throttle_position = parser::parse_throttle_position_response(response);
        }
    }
}
