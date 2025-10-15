use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    configs::CoreConfigs, metadata::metadata_storage::MetadataStorage, utils::clock::Clock,
};

pub mod extend_template_initialization;
pub mod finish_template_initialization;
pub mod mark_template_initialization_as_failed;
pub mod start_template_initialization;

pub struct TemplatesFeature {
    metadata_storage: Arc<MetadataStorage>,
    dbms_connections_pool: PgPool,
    clock: Arc<dyn Clock>,
    configs: Arc<CoreConfigs>,
}

impl TemplatesFeature {
    pub fn new(
        metadata_storage: Arc<MetadataStorage>,
        dbms_connections_pool: PgPool,
        clock: Arc<dyn Clock>,
        configs: Arc<CoreConfigs>,
    ) -> TemplatesFeature {
        TemplatesFeature {
            metadata_storage,
            dbms_connections_pool,
            clock,
            configs,
        }
    }
}
