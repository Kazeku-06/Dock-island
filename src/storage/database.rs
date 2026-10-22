// Database connection and operations - Phase 4

use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = crate::config::database_file_path()?;
        let conn = Connection::open(db_path)?;
        
        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        
        Ok(Self { conn })
    }
    
    pub fn run_migrations(&self) -> Result<()> {
        // Will be implemented in Phase 4
        Ok(())
    }
    
    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}
