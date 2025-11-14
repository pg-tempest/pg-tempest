use std::{env, sync::Arc};

use crate::logging::configs::LoggingConfigs;
use config::{Config, ConfigError};
use pg_tempest_core::configs::template_initialization_configs::TemplateInitializationConfigs;
use pg_tempest_core::configs::{db_pool_configs::DbPoolConfigs, dbms_configs::DbmsConfigs};
use pg_tempest_server::configs::ServerConfigs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfigs {
    pub dbms: Arc<DbmsConfigs>,
    pub db_pool: Arc<DbPoolConfigs>,
    pub server: Arc<ServerConfigs>,
    pub logging: Arc<LoggingConfigs>,
    pub template_initialization: Arc<TemplateInitializationConfigs>,
}

pub fn build_app_configs() -> Result<Arc<AppConfigs>, ConfigError> {
    let configs_path = env::var("PG_TEMPEST_CONFIGS_PATH").unwrap_or("./pg-tempest.toml".into());

    let config = Config::builder()
        .add_source(config::File::with_name("./pg-tempest.defaults.toml").required(true))
        .add_source(config::File::with_name(configs_path.as_str()).required(false))
        .add_source(config::Environment::with_prefix("PG_TEMPEST").separator("_"))
        .build()?;

    config.try_deserialize()
}
