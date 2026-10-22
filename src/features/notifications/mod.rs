// Notifications feature - Phase 11 (P1)

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub app_name: String,
    pub title: String,
    pub body: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub category: NotificationCategory,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NotificationCategory {
    Internal,
    Music,
    External,
}

pub struct NotificationManager {
    // Will be implemented in Phase 11
}

impl NotificationManager {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}
