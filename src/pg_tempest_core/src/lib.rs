use std::sync::Arc;

use anyhow::Ok;
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

use crate::{
    configs::{CoreConfigs, DbmsConfigs},
    features::templates::TemplatesFeature,
    state_manager::StateManager,
    utils::clock::SystemClock,
};

pub mod configs;
pub mod db_queries;
pub mod features;
pub mod models;
pub mod state_manager;
pub mod utils;

pub struct PgTempestCore {
    pub templates_feature: Arc<TemplatesFeature>,
    pub state_manager: Arc<StateManager>,
}

impl PgTempestCore {
    pub async fn new(configs: Arc<CoreConfigs>) -> anyhow::Result<PgTempestCore> {
        let pg_pool = create_pg_pool(&configs.dbms).await?;
        let state_manager = Arc::new(StateManager::new());
        let clock = Arc::new(SystemClock);

        let templates_feature = TemplatesFeature::new(
            state_manager.clone(),
            pg_pool,
            clock.clone(),
            configs.clone(),
        );

        Ok(PgTempestCore {
            templates_feature: Arc::new(templates_feature),
            state_manager: state_manager,
        })
    }
}

async fn create_pg_pool(configs: &DbmsConfigs) -> anyhow::Result<PgPool> {
    let pg_connect_options = PgConnectOptions::new_without_pgpass()
        .host(&configs.host)
        .port(configs.port)
        .database(&configs.database)
        .username(&configs.user)
        .password(&configs.password);

    let pg_pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect_with(pg_connect_options)
        .await?;

    Ok(pg_pool)
}
