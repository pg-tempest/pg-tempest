use std::{collections::VecDeque, time::Duration};

use chrono::{DateTime, Utc};
use tokio::sync::oneshot;

use crate::models::value_types::{template_hash::TemplateHash, test_db_id::TestDbId};

pub struct TemplateMetadata {
    pub template_hash: TemplateHash,
    pub initialization_state: TemplateInitializationState,
    pub test_dbs: Vec<TestDbMetadata>,
    pub test_db_waiters: VecDeque<TestDbWaiter>,
    pub test_db_id_sequence: u16,
}

impl TemplateMetadata {
    pub fn next_test_db_id(&mut self) -> TestDbId {
        self.test_db_id_sequence += 1;
        TestDbId::new(self.test_db_id_sequence)
    }
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
    pub state: TestDbState,
}

pub enum TestDbState {
    Creating,
    Ready,
    Corrupted,
    InUse { usage_deadline: DateTime<Utc> },
}

pub struct TestDbWaiter {
    pub usage_duration: Duration,
    pub readines_sender: oneshot::Sender<TestDbUsage>,
}

pub struct TestDbUsage {
    pub test_db_id: TestDbId,
    pub deadline: DateTime<Utc>,
}
