use crate::models::value_types::{template_hash::TemplateHash, test_db_id::TestDbId};

pub struct TestDb {
    pub template_hash: TemplateHash,
    pub id: TestDbId,
    pub created: bool,
    pub read_only_uses: bool,
}
