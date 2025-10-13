use crate::models::template_database::TemplateDb;
use crate::models::test_db::TestDb;
use crate::models::test_db_usage::TestDbUsage;
use crate::models::value_types::template_hash::TemplateHash;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct StateManager {
    state_sharded_by_template_hash: Mutex<HashMap<TemplateHash, Arc<Mutex<Option<StateShard>>>>>,
}

pub struct StateShard {
    pub template_database: TemplateDb,
    pub test_databases: Vec<TestDb>,
    pub test_database_usages: Vec<TestDbUsage>,
}

pub struct StateShardGuard {}

impl StateManager {
    pub fn new() -> StateManager {
        StateManager {
            state_sharded_by_template_hash: Mutex::new(HashMap::new()),
        }
    }

    pub async fn execute_under_lock<TResult>(
        &self,
        template_hash: TemplateHash,
        action: impl FnOnce(&mut Option<StateShard>) -> TResult,
    ) -> TResult {
        let state_shard = {
            let mut global_state_guard = self.state_sharded_by_template_hash.lock().await;

            global_state_guard
                .entry(template_hash)
                .or_insert_with(|| Arc::new(Mutex::new(None)))
                .clone()
        };

        let mut state_shard = state_shard.lock().await;

        action(&mut *state_shard)
    }
}
