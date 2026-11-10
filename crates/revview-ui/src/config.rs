// Implements settings persistence for desktop platforms using TOML.

use std::{fs, io, path::PathBuf};

use directories::ProjectDirs;
use revview_core::{AppSettings, SettingsStorage};

#[derive(Clone)]
pub struct FileSettingsStorage {
    path: PathBuf,
}

impl FileSettingsStorage {
    pub fn new() -> io::Result<Self> {
        let project_dirs = ProjectDirs::from("com", "RevView", "RevView").ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "Could not determine configuration directory",
            )
        })?;

        Ok(Self {
            path: project_dirs.config_dir().join("config.toml"),
        })
    }
}

impl SettingsStorage for FileSettingsStorage {
    type Error = Box<dyn std::error::Error>;

    fn load(&self) -> Result<AppSettings, Self::Error> {
        // A missing configuration file means first-time startup.
        if !self.path.exists() {
            return Ok(AppSettings::default());
        }

        let contents = fs::read_to_string(&self.path)?;

        let settings = toml::from_str(&contents)?;

        Ok(settings)
    }

    fn save(&self, settings: &AppSettings) -> Result<(), Self::Error> {
        // Ensure the configuration directory exists.
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(settings)?;

        fs::write(&self.path, contents)?;

        Ok(())
    }
}
