// Lyrics display feature - Phase 10 (P1)

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lyrics {
    pub lines: Vec<LyricLine>,
    pub is_synced: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LyricLine {
    pub text: String,
    pub timestamp: Option<u64>, // milliseconds
}

pub struct LyricsProvider {
    // Will be implemented in Phase 10
}

impl LyricsProvider {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}
