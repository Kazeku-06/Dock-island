mod app;
mod config;
mod platform;
mod ui;
mod features;
mod storage;
mod services;

use anyhow::Result;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env()
            .add_directive("topdock=debug".parse()?))
        .init();

    tracing::info!("TopDock starting...");

    // Run the application
    app::run()?;

    tracing::info!("TopDock shutting down");
    Ok(())
}
