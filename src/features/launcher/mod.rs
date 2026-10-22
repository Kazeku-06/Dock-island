// Launcher feature - Phase 5
// Manages application, file, and folder shortcuts

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    pub id: i64,
    pub label: String,
    pub target_path: String,
    pub item_type: ShortcutType,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShortcutType {
    Application,
    File,
    Folder,
}

pub struct LauncherManager {
    // Will be implemented in Phase 5
}

impl LauncherManager {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}
