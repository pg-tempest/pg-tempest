use crate::{
    features::test_dbs::TestDbsFeature,
    models::value_types::{template_hash::TemplateHash, test_db_id::TestDbId},
};

pub enum ReleaseTestDbErrorResult {
    TemplateWasNotFound,
    TestDbWasNotFound,
}

impl TestDbsFeature {
    pub async fn release_test_db(
        &self,
        template_hash: TemplateHash,
        test_db_id: TestDbId,
    ) -> Result<(), ReleaseTestDbErrorResult> {
        self.metadata_storage
            .execute_under_lock(template_hash, |template_metadata| {
                let template_metadata = template_metadata
                    .as_mut()
                    .ok_or(ReleaseTestDbErrorResult::TemplateWasNotFound)?;

                let test_db_metadata = template_metadata
                    .test_dbs
                    .iter_mut()
                    .find(|test_db| test_db.id == test_db_id)
                    .ok_or(ReleaseTestDbErrorResult::TestDbWasNotFound)?;

                test_db_metadata.usage_deadline = None;
                test_db_metadata.corrupted = true;

                Ok(())
            })
            .await
    }
}
