pub mod extend_test_db_usage;
pub mod get_test_db;

use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    configs::CoreConfigs, metadata::metadata_storage::MetadataStorage, utils::clock::Clock,
};

pub struct TestDbsFeature {
    metadata_storage: Arc<MetadataStorage>,
    dbms_connections_pool: PgPool,
    clock: Arc<dyn Clock>,
    configs: Arc<CoreConfigs>,
}

impl TestDbsFeature {
    pub fn new(
        metadata_storage: Arc<MetadataStorage>,
        dbms_connections_pool: PgPool,
        clock: Arc<dyn Clock>,
        configs: Arc<CoreConfigs>,
    ) -> TestDbsFeature {
        TestDbsFeature {
            metadata_storage,
            dbms_connections_pool,
            clock,
            configs,
        }
    }
}
