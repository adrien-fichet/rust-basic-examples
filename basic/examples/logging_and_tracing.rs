// https://tokio.rs/tokio/topics/tracing

use anyhow::Result;
use tracing_subscriber::prelude::*;

fn main() -> Result<()> {
    let stdout_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(true)
        .with_writer(std::io::stdout);

    let log_file = std::fs::File::create("/tmp/logging-and-tracing.log")?;
    let log_file_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(true)
        // .with_ansi(false) // disable ANSI color and style sequences in the log file
        .with_writer(log_file);

    let subscriber = tracing_subscriber::Registry::default()
        .with(stdout_layer)
        .with(log_file_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    tracing::trace!("Trace");
    tracing::debug!("Debug");
    tracing::info!("Info!");
    tracing::warn!("Warning");
    tracing::error!("Error");

    Ok(())
}
