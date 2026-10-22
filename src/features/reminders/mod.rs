// Reminders feature - Phase 8

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub id: i64,
    pub todo_id: Option<i64>,
    pub title: String,
    pub message: Option<String>,
    pub scheduled_at: DateTime<Utc>,
    pub recurrence_rule: Option<String>,
    pub status: ReminderStatus,
    pub snooze_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ReminderStatus {
    Pending,
    Active,
    Snoozed,
    Completed,
    Dismissed,
}

pub struct ReminderManager {
    // Will be implemented in Phase 8
}

impl ReminderManager {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}
