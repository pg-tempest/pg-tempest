use std::sync::Arc;

use crate::{
    PgTempestCore,
    metadata::template_metadata::TestDbState,
    models::value_types::{template_hash::TemplateHash, test_db_id::TestDbId},
};

pub enum ReleaseTestDbErrorResult {
    TemplateWasNotFound,
    TestDbWasNotFound,
    TestDbIsNotInUse,
}

impl PgTempestCore {
    pub async fn release_test_db(
        self: Arc<PgTempestCore>,
        template_hash: TemplateHash,
        test_db_id: TestDbId,
    ) -> Result<(), ReleaseTestDbErrorResult> {
        self.metadata_storage
            .execute_under_lock(template_hash, |template_metadata| {
                let template = template_metadata
                    .as_mut()
                    .ok_or(ReleaseTestDbErrorResult::TemplateWasNotFound)?;

                let test_db = template
                    .test_dbs
                    .iter_mut()
                    .find(|test_db| test_db.id == test_db_id)
                    .ok_or(ReleaseTestDbErrorResult::TestDbWasNotFound)?;

                if !matches!(test_db.state, TestDbState::InUse { .. }) {
                    return Err(ReleaseTestDbErrorResult::TestDbIsNotInUse);
                }

                test_db.state = TestDbState::Creating;

                PgTempestCore::start_test_db_creation_in_background(
                    self.clone(),
                    template_hash,
                    test_db_id,
                );

                Ok(())
            })
            .await
    }
}
