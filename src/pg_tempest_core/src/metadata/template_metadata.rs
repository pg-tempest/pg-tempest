use chrono::{DateTime, Utc};

use crate::models::value_types::{template_hash::TemplateHash, test_db_id::TestDbId};

pub struct TemplateMetadata {
    pub template_hash: TemplateHash,
    pub initialization_state: TemplateInitializationState,
    pub test_dbs: Vec<TestDbMetadata>,
}

pub enum TemplateInitializationState {
    InProgress {
        initialization_deadline: DateTime<Utc>,
    },
    Done,
    Failed,
}

pub struct TestDbMetadata {
    pub id: TestDbId,
    pub usage_deadline: Option<DateTime<Utc>>,
    pub corrupted: bool,
}
