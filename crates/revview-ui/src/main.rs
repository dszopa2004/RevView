// Application entry point. Initializes the UI and starts the application.
mod config;

use std::{
    sync::atomic::Ordering,
    thread,
    time::{Duration, Instant},
};

use config::FileSettingsStorage;
use revview_core::{
    AppSettings, SettingsStorage, SpeedUnit, TemperatureUnit, VehicleTelemetry,
    calculations::{convert_speed, convert_temperature},
    settings::{REDLINE_MAX, REDLINE_MIN},
};

use revview_obd::{
    connection::{initialize_adapter, open_connection, read_response, send_command},
    extras::ExtrasPoller,
    parser,
    scheduler::PollingScheduler,
    vehicles,
};
use std::{
    collections::HashMap,
    sync::{Arc, atomic::AtomicUsize},
};

use revview_core::diagnostics::{CodeKind, CodeStatus, DiagnosticReport};
use revview_obd::commands::codes_command;
use std::sync::mpsc;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let storage = match FileSettingsStorage::new() {
        Ok(storage) => Some(storage),
        Err(e) => {
            eprintln!("Could not initialize settings storage: {}", e);
            None
        }
    };

    let settings = match &storage {
        Some(storage) => match storage.load() {
            Ok(settings) => settings,
            Err(e) => {
                eprintln!("Could not load settings: {}", e);
                AppSettings::default()
            }
        },
        None => AppSettings::default(),
    };

    let app = Dashboard::new()?;

    let vehicle_idx = Arc::new(AtomicUsize::new(vehicles::index_of(&settings.vehicle_id)));
    {
        let profile = vehicles::ALL[vehicle_idx.load(Ordering::Relaxed)];
        app.set_vehicle_index(vehicle_idx.load(Ordering::Relaxed) as i32);
        app.set_hybrid(profile.hybrid);
        let names: Vec<slint::SharedString> = vehicles::ALL.iter().map(|p| p.name.into()).collect();
        app.set_vehicle_names(slint::ModelRc::new(slint::VecModel::from(names)));
    }

    // Apply saved settings once, on the main thread.
    app.set_speed_unit(match settings.speed_unit {
        SpeedUnit::Kph => 0,
        SpeedUnit::Mph => 1,
    });
    app.set_temp_unit(match settings.temperature_unit {
        TemperatureUnit::Celsius => 0,
        TemperatureUnit::Fahrenheit => 1,
    });
    app.set_redline_rpm(settings.redline_rpm.clamp(REDLINE_MIN, REDLINE_MAX) as i32);
    app.set_shift_rpm(settings.shift_rpm.clamp(REDLINE_MIN, REDLINE_MAX) as i32);
    app.set_gauge_style(settings.gauge_style as i32);
    app.set_theme(settings.theme as i32);

    // Create a weak handle to safely pass across thread boundaries
    let app_weak = app.as_weak();

    let (diag_tx, diag_rx) = mpsc::channel::<()>();

    app.on_settings_changed({
        let storage = storage.clone();
        let app_weak = app_weak.clone();

        move || {
            let Some(storage) = &storage else {
                return;
            };

            let Some(ui) = app_weak.upgrade() else {
                return;
            };

            let settings = AppSettings {
                speed_unit: match ui.get_speed_unit() {
                    1 => SpeedUnit::Mph,
                    _ => SpeedUnit::Kph,
                },

                temperature_unit: match ui.get_temp_unit() {
                    1 => TemperatureUnit::Fahrenheit,
                    _ => TemperatureUnit::Celsius,
                },
                redline_rpm: (ui.get_redline_rpm().max(0) as u32).clamp(REDLINE_MIN, REDLINE_MAX),
                shift_rpm: (ui.get_shift_rpm().max(0) as u32).clamp(REDLINE_MIN, REDLINE_MAX),
                gauge_style: (ui.get_gauge_style().max(0) as u32),
                theme: ui.get_theme().max(0) as u32,
                vehicle_id: vehicles::ALL
                    [(ui.get_vehicle_index().max(0) as usize).min(vehicles::ALL.len() - 1)]
                .id
                .to_string(),
            };

            if let Err(e) = storage.save(&settings) {
                eprintln!("Could not save settings: {}", e);
            }
        }
    });

    app.on_vehicle_changed({
        let app_weak = app_weak.clone();
        let vehicle_idx = vehicle_idx.clone();
        move |i| {
            let i = (i.max(0) as usize).min(vehicles::ALL.len() - 1);
            vehicle_idx.store(i, Ordering::Relaxed);
            let profile = vehicles::ALL[i];

            if let Some(ui) = app_weak.upgrade() {
                ui.set_redline_rpm(profile.redline_default as i32);
                ui.set_shift_rpm(profile.shift_default as i32);
                ui.set_hybrid(profile.hybrid);
                ui.set_extras(slint::ModelRc::new(slint::VecModel::from(Vec::new())));
                ui.set_current_view(0); // the extras view may disappear
                ui.invoke_settings_changed(); // save
            }
        }
    });

    app.on_run_diagnostics({
        let app_weak = app_weak.clone();
        move || {
            // If the OBD thread has exited (no adapter), nobody is listening.
            if diag_tx.send(()).is_err()
                && let Some(ui) = app_weak.upgrade()
            {
                ui.set_diag_state(3);
            }
        }
    });

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

                    update_ui(&app_weak, move |ui| {
                        ui.set_obd_initialized(true);
                    });
                    return;
                }

                update_ui(&app_weak, move |ui| {
                    ui.set_obd_initialized(true);
                });

                println!("Adapter initialized successfully!");

                // Infinite loop to continuously update telemetry while the app runs

                let mut extras = ExtrasPoller::new(Duration::from_millis(500));
                let mut extra_values: HashMap<&'static str, f32> = HashMap::new();
                let mut report = DiagnosticReport::default(); // fresh each connection, never saved
                let mut scheduler = PollingScheduler::new();

                loop {
                    if diag_rx.try_recv().is_ok() {
                        run_diagnostics(&mut *port, &mut report, &app_weak);
                        continue;
                    }

                    let profile = vehicles::ALL[vehicle_idx.load(Ordering::Relaxed)];
                    if !profile.extra_pids.is_empty() && extras.due(Instant::now()) {
                        extras.poll_next(&mut *port, profile, &mut extra_values);

                        let rows: Vec<(String, String, String)> = profile
                            .extra_pids
                            .iter()
                            .filter_map(|p| {
                                extra_values.get(p.id).map(|v| {
                                    (p.label.to_string(), format!("{v:.1}"), p.unit.to_string())
                                })
                            })
                            .collect();

                        update_ui(&app_weak, move |ui| {
                            let model: Vec<ExtraReading> = rows
                                .into_iter()
                                .map(|(label, value, unit)| ExtraReading {
                                    label: label.into(),
                                    value: value.into(),
                                    unit: unit.into(),
                                })
                                .collect();
                            ui.set_extras(slint::ModelRc::new(slint::VecModel::from(model)));
                        });
                    }

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
                        let speed_kph = telemetry.speed_kph.unwrap_or(0);
                        let coolant_temp_c = telemetry.coolant_temp_c.unwrap_or(0);
                        let intake_temp_c = telemetry.intake_temp_c.unwrap_or(0);
                        let throttle_position = telemetry.throttle_position.unwrap_or(0.0);

                        update_ui(&app_weak, move |ui| {
                            // Read the currently selected units from the UI.
                            let speed_unit = match ui.get_speed_unit() {
                                1 => SpeedUnit::Mph,
                                _ => SpeedUnit::Kph,
                            };

                            let temperature_unit = match ui.get_temp_unit() {
                                1 => TemperatureUnit::Fahrenheit,
                                _ => TemperatureUnit::Celsius,
                            };

                            let speed = convert_speed(speed_kph, speed_unit);
                            let coolant_temp =
                                convert_temperature(coolant_temp_c, temperature_unit);
                            let intake_temp = convert_temperature(intake_temp_c, temperature_unit);

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

fn run_diagnostics(
    port: &mut dyn serialport::SerialPort,
    report: &mut DiagnosticReport,
    app_weak: &slint::Weak<Dashboard>,
) {
    report.clear();

    for kind in [CodeKind::Stored, CodeKind::Pending, CodeKind::Permanent] {
        report.set(kind, CodeStatus::Requesting);

        let status = match request_telemetry(port, codes_command(kind)) {
            Ok(response) => {
                let parsed = match kind {
                    CodeKind::Stored => parser::parse_stored_codes_response(&response),
                    CodeKind::Pending => parser::parse_pending_codes_response(&response),
                    CodeKind::Permanent => parser::parse_permanent_codes_response(&response),
                };
                match parsed {
                    Some(codes) => CodeStatus::Loaded(codes),
                    None => {
                        eprintln!("{:?} codes: unparseable response: {:?}", kind, response);
                        CodeStatus::Failed
                    }
                }
            }
            Err(e) => {
                eprintln!("{:?} codes request failed: {}", kind, e);
                CodeStatus::Failed
            }
        };

        report.set(kind, status);
    }

    // Done if at least one kind loaded. Older cars often don't support
    // mode 0A, and that shouldn't fail the whole scan.
    let any_loaded = [&report.stored, &report.pending, &report.permanent]
        .iter()
        .any(|s| matches!(s, CodeStatus::Loaded(_)));

    let mut entries: Vec<(String, &'static str)> = Vec::new();
    for (status, label) in [
        (&report.stored, "Stored"),
        (&report.pending, "Pending"),
        (&report.permanent, "Permanent"),
    ] {
        if let CodeStatus::Loaded(codes) = status {
            entries.extend(codes.iter().map(|c| (c.0.clone(), label)));
        }
    }

    update_ui(app_weak, move |ui| {
        let model: Vec<DiagnosticEntry> = entries
            .into_iter()
            .map(|(code, kind)| DiagnosticEntry {
                code: code.into(),
                kind: kind.into(),
            })
            .collect();

        // Entries first, then state, so the list is never shown empty.
        ui.set_diag_entries(slint::ModelRc::new(slint::VecModel::from(model)));
        ui.set_diag_state(if any_loaded { 2 } else { 3 });
    });
}
