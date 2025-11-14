use std::sync::Arc;

use anyhow::Ok;

use crate::{
    configs::{db_pool_configs::DbPoolConfigs, dbms_configs::DbmsConfigs},
    metadata::metadata_storage::MetadataStorage,
    pg_client::PgClient,
    utils::clock::{Clock, SystemClock},
};
use crate::configs::template_initialization_configs::TemplateInitializationConfigs;

pub mod configs;
pub mod features;
pub mod metadata;
pub mod models;
pub mod pg_client;
pub mod pg_client_extensions;
pub mod utils;

pub struct PgTempestCore {
    metadata_storage: Arc<MetadataStorage>,
    clock: Arc<dyn Clock>,
    pg_client: Arc<dyn PgClient>,
    dbms_configs: Arc<DbmsConfigs>,
    db_pool_configs: Arc<DbPoolConfigs>,
    template_initialization_configs: Arc<TemplateInitializationConfigs>,
}

impl PgTempestCore {
    pub async fn new(
        pg_client: Arc<dyn PgClient>,
        dbms_configs: Arc<DbmsConfigs>,
        db_pool_configs: Arc<DbPoolConfigs>,
        template_initialization_configs: Arc<TemplateInitializationConfigs>,
    ) -> anyhow::Result<PgTempestCore> {
        let metadata_storage = Arc::new(MetadataStorage::new());
        let clock = Arc::new(SystemClock);

        Ok(PgTempestCore {
            metadata_storage,
            clock,
            pg_client,
            db_pool_configs,
            dbms_configs,
            template_initialization_configs,
        })
    }
}
