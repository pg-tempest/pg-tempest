use std::time::Duration;

use chrono::{DateTime, Utc};

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
    TestDbIsNotInUse,
    TestDbIsCorrupted,
}

impl PgTempestCore {
    pub async fn extend_test_db_usage(
        &self,
        template_hash: TemplateHash,
        test_db_id: TestDbId,
        additional_time: Duration,
    ) -> Result<ExtendTestDbUsageOkResult, ExtendTestDbUsageErrorResult> {
        self.metadata_storage
            .execute_under_lock(template_hash, |template_metadata| {
                let Some(template_metadata) = template_metadata else {
                    return Err(ExtendTestDbUsageErrorResult::TemplateWasNotFound);
                };

                let Some(test_db) = template_metadata
                    .test_dbs
                    .iter_mut()
                    .find(|test_db| test_db.id == test_db_id)
                else {
                    return Err(ExtendTestDbUsageErrorResult::TestDbWasNotFound);
                };

                let TestDbState::InUse {
                    ref mut usage_deadline,
                } = test_db.state
                else {
                    return Err(ExtendTestDbUsageErrorResult::TestDbIsNotInUse);
                };

                *usage_deadline = *usage_deadline + additional_time;

                Ok(ExtendTestDbUsageOkResult {
                    new_usage_deadline: *usage_deadline,
                })
            })
            .await
    }
}
