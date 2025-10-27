use std::sync::Arc;

use tracing::{info, instrument, warn};

use crate::{
    PgTempestCore,
    metadata::template_metadata::TestDbState,
    models::value_types::{template_hash::TemplateHash, test_db_id::TestDbId},
};

pub enum FinishTestDbUsageErrorResult {
    TemplateWasNotFound,
    TestDbWasNotFound,
    TestDbIsNotUsed,
}

impl PgTempestCore {
    #[instrument(skip_all)]
    pub async fn finish_test_db_usage(
        self: Arc<PgTempestCore>,
        template_hash: TemplateHash,
        test_db_id: TestDbId,
    ) -> Result<(), FinishTestDbUsageErrorResult> {
        self.metadata_storage
            .execute_under_lock(template_hash, |template| {
                let Some(template) = template else {
                    warn!("Template {template_hash} was not found");
                    return Err(FinishTestDbUsageErrorResult::TemplateWasNotFound);
                };

                let test_db = template
                    .test_dbs
                    .iter_mut()
                    .find(|test_db| test_db.id == test_db_id);

                let Some(test_db) = test_db else {
                    warn!("Test db {template_hash} {test_db_id} was not found");
                    return Err(FinishTestDbUsageErrorResult::TestDbWasNotFound);
                };

                if !matches!(test_db.state, TestDbState::InUse { .. }) {
                    warn!("Test db {template_hash} {test_db_id} is not used");
                    return Err(FinishTestDbUsageErrorResult::TestDbIsNotUsed);
                }

                test_db.state = TestDbState::Creating;

                tokio::spawn(self.clone().recreate_test_db(template_hash, test_db_id));

                info!("Test db {template_hash} {test_db_id} usage was finished");

                Ok(())
            })
            .await
    }
}
