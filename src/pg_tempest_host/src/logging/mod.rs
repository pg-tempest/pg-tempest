use std::sync::Arc;

use anyhow::Ok;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;

use crate::logging::configs::LoggingConfigs;

pub mod configs;

pub fn setup_logging(configs: Arc<LoggingConfigs>) -> anyhow::Result<()> {
    let filter = tracing_subscriber::filter::Targets::new()
        .with_default(Level::INFO)
        .with_target("pg_tempest_core", configs.core.unwrap_or(Level::INFO))
        .with_target("pg_tempest_server", configs.server.unwrap_or(Level::INFO))
        .with_target("sqlx", configs.db_queries.unwrap_or(Level::INFO));

    let r = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .with(filter);

    tracing::subscriber::set_global_default(r)?;

    Ok(())
}
