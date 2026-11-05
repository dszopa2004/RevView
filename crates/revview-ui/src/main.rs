// Application entry point. Initializes the UI and starts the application.

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = Dashboard::new()?;
    app.run()
}
