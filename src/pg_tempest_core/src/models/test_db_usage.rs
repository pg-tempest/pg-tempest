use chrono::{DateTime, Utc};

use crate::models::value_types::{template_hash::TemplateHash, test_db_id::TestDbId};

pub type TestDbUsageId = u32;

pub struct TestDbUsage {
    pub template_hash: TemplateHash,
    pub test_database_id: TestDbId,
    pub is_read_only: bool,
    pub started_at: DateTime<Utc>,
    pub deadline_at: DateTime<Utc>,
}
