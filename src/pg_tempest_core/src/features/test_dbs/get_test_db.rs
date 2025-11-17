use chrono::{DateTime, Utc};
use std::{sync::Arc, time::Duration};
use tokio::sync::oneshot;
use tracing::{debug, info, instrument, warn};

use crate::utils::unexpected_error::UnexpectedError;
use crate::{
    PgTempestCore,
    metadata::template_metadata::{
        TemplateInitializationState, TestDbAwaiter, TestDbMetadata, TestDbState, TestDbUsage,
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
    TemplateIsNotInitialized,
    Unknown { inner: UnexpectedError },
}

impl PgTempestCore {
    #[instrument(skip_all)]
    pub async fn get_test_db(
        self: Arc<PgTempestCore>,
        template_hash: TemplateHash,
        usage_duration: Duration,
    ) -> Result<GetTestDbOkResult, GetTestDbErrorResult> {
        let test_db_usage_or_receiver: TestDbUsageOrReceiver = self
            .metadata_storage
            .execute_under_lock(template_hash, |template| {
                let Some(template) = template else {
                    warn!("Template {template_hash} was not found");
                    return Err(GetTestDbErrorResult::TemplateWasNotFound);
                };

                if !matches!(
                    template.initialization_state,
                    TemplateInitializationState::Finished
                ) {
                    warn!("Template {template_hash} initialization is not finished");
                    return Err(GetTestDbErrorResult::TemplateIsNotInitialized);
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

                    return Ok(TestDbUsageOrReceiver::Usage(usage));
                }

                debug!("Ready test db {template_hash} was not found in pool");

                let (sender, receiver) = oneshot::channel();
                let awaiter = TestDbAwaiter {
                    usage_duration,
                    readiness_sender: sender,
                };
                template.test_db_awaiters.push_back(awaiter);

                let test_dbs_in_creation = template
                    .test_dbs
                    .iter()
                    .filter(|x| matches!(x.state, TestDbState::Creating))
                    .count();
                let awaiters_count = template.test_db_awaiters.len();

                if awaiters_count > test_dbs_in_creation {
                    let test_db_id = template.next_test_db_id();
                    let test_db = TestDbMetadata {
                        id: test_db_id,
                        state: TestDbState::Creating,
                    };

                    tokio::spawn(self.clone().recreate_test_db(template_hash, test_db_id));

                    template.test_dbs.push(test_db);

                    info!("New test db {template_hash} {test_db_id} was added to pool");
                }

                Ok(TestDbUsageOrReceiver::Receiver(receiver))
            })
            .await?;

        let usage = match test_db_usage_or_receiver {
            TestDbUsageOrReceiver::Usage(usage) => usage,
            // TODO: Remove unwrap
            TestDbUsageOrReceiver::Receiver(receiver) => receiver.await.unwrap(),
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

enum TestDbUsageOrReceiver {
    Usage(TestDbUsage),
    Receiver(oneshot::Receiver<TestDbUsage>),
}
