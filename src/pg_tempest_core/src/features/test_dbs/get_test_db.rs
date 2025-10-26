use std::{sync::Arc, time::Duration};

use chrono::{DateTime, Utc};
use tokio::sync::oneshot;
use tracing::{debug, info, instrument, warn};

use crate::{
    PgTempestCore,
    metadata::template_metadata::{
        TemplateInitializationState, TestDbMetadata, TestDbState, TestDbUsage, TestDbWaiter,
    },
    models::{
        db_connection_options::DbConnectionOptions,
        value_types::{
            template_hash::TemplateHash, test_db_id::TestDbId, test_db_name::TestDbName,
        },
    },
};

pub struct GetTestDbOkResult {
    pub test_db_id: TestDbId,
    pub connection_options: DbConnectionOptions,
    pub usage_deadline: DateTime<Utc>,
}

pub enum GetTestDbErrorResult {
    TemplateWasNotFound,
    TemplateIsNotInitalized,
    Unknown { inner: anyhow::Error },
}

impl PgTempestCore {
    #[instrument(skip_all)]
    pub async fn get_test_db(
        self: Arc<PgTempestCore>,
        template_hash: TemplateHash,
        usage_duration: Duration,
    ) -> Result<GetTestDbOkResult, GetTestDbErrorResult> {
        let test_db_usage_or_reciver: TestDbUsageOrReciver = self
            .metadata_storage
            .execute_under_lock(template_hash, |template| {
                let Some(template) = template else {
                    warn!("Template {template_hash} was not found");
                    return Err(GetTestDbErrorResult::TemplateWasNotFound);
                };

                if !matches!(
                    template.initialization_state,
                    TemplateInitializationState::Done
                ) {
                    warn!("Template {template_hash} initialization is not finished");
                    return Err(GetTestDbErrorResult::TemplateIsNotInitalized);
                };

                let ready_test_db = template
                    .test_dbs
                    .iter_mut()
                    .find(|test_db| matches!(test_db.state, TestDbState::Ready));

                if let Some(ready_test_db) = ready_test_db {
                    let test_db_id = ready_test_db.id;
                    let usage = TestDbUsage {
                        test_db_id,
                        deadline: self.clock.now() + usage_duration,
                    };

                    ready_test_db.state = TestDbState::InUse {
                        usage_deadline: usage.deadline,
                    };

                    debug!("Ready test db {template_hash} {test_db_id} was get from pool");

                    return Ok(TestDbUsageOrReciver::Usage(usage));
                }

                debug!("Ready test db {template_hash} was not found in pool");

                let (sender, reciver) = oneshot::channel();
                let waiter = TestDbWaiter {
                    usage_duration,
                    readines_sender: sender,
                };
                template.test_db_waiters.push_back(waiter);

                let test_dbs_in_creation = template
                    .test_dbs
                    .iter()
                    .filter(|x| matches!(x.state, TestDbState::Creating))
                    .count();
                let waiter_count = template.test_db_waiters.len();

                if waiter_count > test_dbs_in_creation {
                    let test_db_id = template.next_test_db_id();
                    let test_db = TestDbMetadata {
                        id: test_db_id,
                        state: TestDbState::Creating,
                    };

                    tokio::spawn(self.clone().recreate_test_db(template_hash, test_db_id));

                    template.test_dbs.push(test_db);

                    info!("New test db {template_hash} {test_db_id} was added to pool");
                }

                Ok(TestDbUsageOrReciver::Reciver(reciver))
            })
            .await?;

        let usage = match test_db_usage_or_reciver {
            TestDbUsageOrReciver::Usage(usage) => usage,
            TestDbUsageOrReciver::Reciver(reciver) => reciver.await.unwrap(),
        };

        info!(
            "Test db {template_hash} {} usage was started",
            usage.test_db_id
        );

        let test_db_name = TestDbName::new(template_hash, usage.test_db_id);

        Ok(GetTestDbOkResult {
            test_db_id: usage.test_db_id,
            connection_options: DbConnectionOptions::new_outer(
                &self.dbms_configs,
                test_db_name.into(),
            ),
            usage_deadline: usage.deadline,
        })
    }
}

enum TestDbUsageOrReciver {
    Usage(TestDbUsage),
    Reciver(oneshot::Receiver<TestDbUsage>),
}
