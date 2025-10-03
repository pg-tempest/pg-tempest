use crate::models::template_database::TemplateHash;
use crate::models::test_database::TestDatabaseId;
use chrono::{DateTime, Utc};

pub type TestDatabaseUsageId = u32;

pub struct TestDatabaseUsage {
    pub template_hash: TemplateHash,
    pub test_database_id: TestDatabaseId,
    pub is_read_only: bool,
    pub started_at: DateTime<Utc>,
    pub deadline_at: DateTime<Utc>,
}
