use std::{collections::VecDeque, time::Duration};

use chrono::{DateTime, Utc};
use tokio::sync::oneshot;

use crate::models::value_types::{template_hash::TemplateHash, test_db_id::TestDbId};

pub struct TemplateMetadata {
    pub template_hash: TemplateHash,
    pub initialization_state: TemplateInitializationState,
    pub template_awaiters: VecDeque<TemplateAwaiter>,
    pub test_dbs: Vec<TestDbMetadata>,
    pub test_db_awaiters: VecDeque<TestDbAwaiter>,
    pub test_db_id_sequence: u16,
}

impl TemplateMetadata {
    pub fn next_test_db_id(&mut self) -> TestDbId {
        self.test_db_id_sequence += 1;
        TestDbId::new(self.test_db_id_sequence)
    }
}

pub enum TemplateInitializationState {
    Creating,
    Created,
    InProgress {
        initialization_deadline: DateTime<Utc>,
    },
    Finished,
    Failed,
}

pub struct TemplateAwaiter {
    pub initialization_duration: Duration,
    pub result_sender: oneshot::Sender<TemplateAwaitingResult>,
}

pub enum TemplateAwaitingResult {
    InitializationIsStarted {
        initialization_deadline: DateTime<Utc>,
    },
    InitializationIsInProgress,
    InitializationIsFinished,
    InitializationIsFailed,
    FailedToCreateTemplateDb,
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

pub struct TestDbAwaiter {
    pub usage_duration: Duration,
    pub readiness_sender: oneshot::Sender<TestDbUsage>,
}

pub struct TestDbUsage {
    pub test_db_id: TestDbId,
    pub deadline: DateTime<Utc>,
}
