// Database migrations - Phase 4

use anyhow::Result;
use rusqlite::Connection;

pub struct Migration {
    pub version: i32,
    pub name: &'static str,
    pub sql: &'static str,
}

pub const MIGRATIONS: &[Migration] = &[
    // Migrations will be added in Phase 4
];

pub fn run_migrations(conn: &Connection) -> Result<()> {
    // Create migrations table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS migrations (
            version INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    
    // Get current version
    let current_version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM migrations",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);
    
    // Apply pending migrations
    for migration in MIGRATIONS {
        if migration.version > current_version {
            tracing::info!("Applying migration {}: {}", migration.version, migration.name);
            conn.execute_batch(migration.sql)?;
            conn.execute(
                "INSERT INTO migrations (version, name) VALUES (?1, ?2)",
                rusqlite::params![migration.version, migration.name],
            )?;
        }
    }
    
    Ok(())
}
