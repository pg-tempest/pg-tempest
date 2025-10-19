use std::sync::Arc;

use anyhow::Ok;
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

use crate::{
    configs::{CoreConfigs, DbmsConfigs},
    features::{templates::TemplatesFeature, test_dbs::TestDbsFeature},
    metadata::metadata_storage::MetadataStorage,
    utils::clock::SystemClock,
};

pub mod configs;
pub mod db_queries;
pub mod features;
pub mod metadata;
pub mod models;
pub mod utils;

pub struct PgTempestCore {
    pub templates_feature: Arc<TemplatesFeature>,
    pub test_dbs_feature: Arc<TestDbsFeature>,
    pub metadata_storage: Arc<MetadataStorage>,
}

impl PgTempestCore {
    pub async fn new(configs: Arc<CoreConfigs>) -> anyhow::Result<PgTempestCore> {
        let pg_pool = create_pg_pool(&configs.dbms).await?;
        let metadata_storage = Arc::new(MetadataStorage::new());
        let clock = Arc::new(SystemClock);

        let templates_feature = TemplatesFeature::new(
            metadata_storage.clone(),
            pg_pool.clone(),
            clock.clone(),
            configs.clone(),
        );

        let test_dbs_feature = TestDbsFeature::new(
            metadata_storage.clone(),
            pg_pool.clone(),
            clock.clone(),
            configs.clone(),
        );

        Ok(PgTempestCore {
            templates_feature: Arc::new(templates_feature),
            test_dbs_feature: Arc::new(test_dbs_feature),
            metadata_storage,
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
