use chrono::{DateTime, Utc};

use crate::models::value_types::template_hash::TemplateHash;

#[derive(Clone)]
pub struct TemplateDb {
    pub hash: TemplateHash,
    pub initialization_state: TemplateInitializationState,
}

#[derive(Clone)]
pub enum TemplateInitializationState {
    InProgress {
        initialization_deadline: DateTime<Utc>,
    },
    Done,
    Failed,
}
