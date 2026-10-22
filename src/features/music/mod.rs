// Music control feature - Phase 6

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub artwork_url: Option<String>,
    pub duration: Option<u64>,
    pub position: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

pub struct MusicController {
    // Will be implemented in Phase 6
}

impl MusicController {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}
