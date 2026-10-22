mod state;

pub use state::AppState;

/// UI state enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiMode {
    CompactIdle,
    Hovered,
    Expanded,
    MusicActive,
    ReminderAlert,
    NotificationPreview,
    DropTarget,
    SettingsOpen,
}

impl Default for UiMode {
    fn default() -> Self {
        UiMode::CompactIdle
    }
}
