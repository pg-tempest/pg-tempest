use crate::models::template_database::TemplateHash;

pub struct TestDatabase {
    pub template_hash: TemplateHash,
    pub id: TestDatabaseId,
    pub created: bool,
    pub read_only_uses: bool,
}

pub type TestDatabaseId = u32;