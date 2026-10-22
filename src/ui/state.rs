use super::UiMode;

/// Application UI state
pub struct AppState {
    pub mode: UiMode,
    pub is_hovered: bool,
    pub active_panel: Option<ActivePanel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanel {
    Launcher,
    Music,
    Todo,
    Reminders,
    Notifications,
    Settings,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            mode: UiMode::default(),
            is_hovered: false,
            active_panel: None,
        }
    }
    
    pub fn set_mode(&mut self, mode: UiMode) {
        tracing::debug!("UI mode changed: {:?} -> {:?}", self.mode, mode);
        self.mode = mode;
    }
    
    pub fn open_panel(&mut self, panel: ActivePanel) {
        tracing::debug!("Opening panel: {:?}", panel);
        self.active_panel = Some(panel);
        self.mode = UiMode::Expanded;
    }
    
    pub fn close_panel(&mut self) {
        tracing::debug!("Closing panel");
        self.active_panel = None;
        self.mode = if self.is_hovered {
            UiMode::Hovered
        } else {
            UiMode::CompactIdle
        };
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
