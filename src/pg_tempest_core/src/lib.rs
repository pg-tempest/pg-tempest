use std::sync::Arc;

use anyhow::Ok;
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

use crate::{
    configs::{db_pool_configs::DbPoolConfigs, dbms_configs::DbmsConfigs},
    metadata::metadata_storage::MetadataStorage,
    utils::clock::{Clock, SystemClock},
};

pub mod configs;
pub mod db_queries;
pub mod features;
pub mod metadata;
pub mod models;
pub mod utils;

pub struct PgTempestCore {
    metadata_storage: Arc<MetadataStorage>,
    dbms_connections_pool: PgPool,
    clock: Arc<dyn Clock>,
    dbms_configs: Arc<DbmsConfigs>,
    db_pool_configs: Arc<DbPoolConfigs>,
}

impl PgTempestCore {
    pub async fn new(
        dbms_configs: Arc<DbmsConfigs>,
        db_pool_configs: Arc<DbPoolConfigs>,
    ) -> anyhow::Result<PgTempestCore> {
        let pg_pool = create_pg_pool(&dbms_configs).await?;
        let metadata_storage = Arc::new(MetadataStorage::new());
        let clock = Arc::new(SystemClock);

        Ok(PgTempestCore {
            metadata_storage,
            dbms_connections_pool: pg_pool,
            clock,
            db_pool_configs,
            dbms_configs,
        })
    }
}

async fn create_pg_pool(configs: &DbmsConfigs) -> anyhow::Result<PgPool> {
    let pg_connect_options = PgConnectOptions::new_without_pgpass()
        .host(&configs.inner.host)
        .port(configs.inner.port)
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
