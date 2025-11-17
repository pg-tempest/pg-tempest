use std::sync::Arc;

use crate::configs::templates_configs::TemplatesConfigs;
use crate::utils::unexpected_error::UnexpectedError;
use crate::{
    configs::{db_pool_configs::DbPoolConfigs, dbms_configs::DbmsConfigs},
    metadata::metadata_storage::MetadataStorage,
    pg_client::PgClient,
    utils::clock::{Clock, SystemClock},
};

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
    templates_configs: Arc<TemplatesConfigs>,
}

impl PgTempestCore {
    pub async fn new(
        pg_client: Arc<dyn PgClient>,
        dbms_configs: Arc<DbmsConfigs>,
        db_pool_configs: Arc<DbPoolConfigs>,
        templates_configs: Arc<TemplatesConfigs>,
    ) -> Result<PgTempestCore, UnexpectedError> {
        let metadata_storage = Arc::new(MetadataStorage::new());
        let clock = Arc::new(SystemClock);

        Ok(PgTempestCore {
            metadata_storage,
            clock,
            pg_client,
            db_pool_configs,
            dbms_configs,
            templates_configs,
        })
    }
}
