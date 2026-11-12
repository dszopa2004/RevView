// Defines the interface used by platform-specific settings storage implementations.

use crate::settings::AppSettings;

pub trait SettingsStorage {
    type Error;

    // Load application settings from storage.
    fn load(&self) -> Result<AppSettings, Self::Error>;

    // Save application settings to storage.
    fn save(&self, settings: &AppSettings) -> Result<(), Self::Error>;
}
