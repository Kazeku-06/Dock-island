mod settings;

pub use settings::Config;

use anyhow::Result;
use std::path::PathBuf;

/// Get the application data directory
pub fn app_data_dir() -> Result<PathBuf> {
    let home = std::env::var("USERPROFILE")?;
    let app_dir = PathBuf::from(home).join(".topdock");
    
    if !app_dir.exists() {
        std::fs::create_dir_all(&app_dir)?;
    }
    
    Ok(app_dir)
}

/// Get the config file path
pub fn config_file_path() -> Result<PathBuf> {
    Ok(app_data_dir()?.join("config.toml"))
}

/// Get the database file path
pub fn database_file_path() -> Result<PathBuf> {
    Ok(app_data_dir()?.join("topdock.db"))
}
