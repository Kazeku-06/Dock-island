use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Compact mode width in pixels
    pub compact_width: u32,
    
    /// Compact mode height in pixels
    pub compact_height: u32,
    
    /// Expanded mode width in pixels
    pub expanded_width: u32,
    
    /// Expanded mode height in pixels
    pub expanded_height: u32,
    
    /// Distance from top of screen in pixels
    pub top_margin: u32,
    
    /// Horizontal alignment: "left", "center", "right"
    pub alignment: String,
    
    /// Enable animations
    pub animations_enabled: bool,
    
    /// Animation duration in milliseconds
    pub animation_duration: u32,
    
    /// Always on top
    pub always_on_top: bool,
    
    /// Start on login
    pub start_on_login: bool,
    
    /// Theme: "dark", "light", "system"
    pub theme: String,
    
    /// Monitor index (0 for primary)
    pub monitor_index: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            compact_width: 200,
            compact_height: 40,
            expanded_width: 400,
            expanded_height: 300,
            top_margin: 10,
            alignment: "center".to_string(),
            animations_enabled: true,
            animation_duration: 200,
            always_on_top: true,
            start_on_login: false,
            theme: "dark".to_string(),
            monitor_index: 0,
        }
    }
}

impl Config {
    /// Load configuration from file or create default
    pub fn load() -> Result<Self> {
        let config_path = super::config_file_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            tracing::info!("Loaded config from {:?}", config_path);
            Ok(config)
        } else {
            let config = Config::default();
            config.save()?;
            tracing::info!("Created default config at {:?}", config_path);
            Ok(config)
        }
    }
    
    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_path = super::config_file_path()?;
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        tracing::info!("Saved config to {:?}", config_path);
        Ok(())
    }
}
