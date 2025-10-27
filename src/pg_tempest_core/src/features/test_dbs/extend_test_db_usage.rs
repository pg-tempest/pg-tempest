use std::time::Duration;

use chrono::{DateTime, Utc};
use tracing::{info, instrument, warn};

use crate::{
    PgTempestCore,
    metadata::template_metadata::TestDbState,
    models::value_types::{template_hash::TemplateHash, test_db_id::TestDbId},
};

pub struct ExtendTestDbUsageOkResult {
    pub new_usage_deadline: DateTime<Utc>,
}

pub enum ExtendTestDbUsageErrorResult {
    TemplateWasNotFound,
    TestDbWasNotFound,
    TestDbIsNotUsed,
    TestDbIsCorrupted,
}

impl PgTempestCore {
    #[instrument(skip_all)]
    pub async fn extend_test_db_usage(
        &self,
        template_hash: TemplateHash,
        test_db_id: TestDbId,
        additional_time: Duration,
    ) -> Result<ExtendTestDbUsageOkResult, ExtendTestDbUsageErrorResult> {
        self.metadata_storage
            .execute_under_lock(template_hash, |template_metadata| {
                let Some(template) = template_metadata else {
                    warn!("Template {template_hash} was not found");
                    return Err(ExtendTestDbUsageErrorResult::TemplateWasNotFound);
                };

                let test_db = template
                    .test_dbs
                    .iter_mut()
                    .find(|test_db| test_db.id == test_db_id);

                let Some(test_db) = test_db else {
                    warn!("Test db {template_hash} {test_db_id} was not found");
                    return Err(ExtendTestDbUsageErrorResult::TestDbWasNotFound);
                };

                let TestDbState::InUse {
                    ref mut usage_deadline,
                } = test_db.state
                else {
                    warn!("Test db {template_hash} {test_db_id} is not used");
                    return Err(ExtendTestDbUsageErrorResult::TestDbIsNotUsed);
                };

                *usage_deadline = *usage_deadline + additional_time;

                info!(
                    "Test db {template_hash} {test_db_id} usage deadline was extended by {} ms",
                    additional_time.as_millis()
                );

                Ok(ExtendTestDbUsageOkResult {
                    new_usage_deadline: *usage_deadline,
                })
            })
            .await
    }
}
