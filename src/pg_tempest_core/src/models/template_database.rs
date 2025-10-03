use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct TemplateDatabase {
    pub hash: TemplateHash,
    pub has_idempotent_initialization: bool,
    pub initialization_attempts: u32,
    pub initialized: bool,
    pub initialization_deadline_at: DateTime<Utc>,
}

pub type TemplateHash = Box<[u8]>;