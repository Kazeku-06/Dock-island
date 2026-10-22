// Database and storage management - Phase 4

mod database;
pub mod migrations;

pub use database::Database;

use anyhow::Result;

pub fn init_storage() -> Result<Database> {
    let db = Database::new()?;
    db.run_migrations()?;
    Ok(db)
}
