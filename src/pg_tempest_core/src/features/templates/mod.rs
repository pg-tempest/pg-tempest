use std::sync::Arc;

use sqlx::PgPool;

use crate::{configs::CoreConfigs, state_manager::StateManager, utils::clock::Clock};

pub mod finish_template_initialization;
pub mod mark_template_initialization_as_failed;
pub mod start_template_initialization;

pub struct TemplatesFeature {
    state_manager: Arc<StateManager>,
    dbms_connections_pool: PgPool,
    clock: Arc<dyn Clock>,
    configs: Arc<CoreConfigs>,
}

impl TemplatesFeature {
    pub fn new(
        state_manager: Arc<StateManager>,
        dbms_connections_pool: PgPool,
        clock: Arc<dyn Clock>,
        configs: Arc<CoreConfigs>,
    ) -> TemplatesFeature {
        TemplatesFeature {
            state_manager,
            dbms_connections_pool,
            clock,
            configs,
        }
    }
}
