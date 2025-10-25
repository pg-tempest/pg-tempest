use std::sync::Arc;

use crate::{
    PgTempestCore,
    metadata::template_metadata::{TemplateInitializationState, TestDbMetadata, TestDbState},
    models::value_types::template_hash::TemplateHash,
};

pub enum FinishTemplateInitializationErrorResult {
    TemplateWasNotFound,
    InitializationIsFailed,
}

impl PgTempestCore {
    pub async fn finish_template_initialization(
        self: Arc<PgTempestCore>,
        template_hash: TemplateHash,
    ) -> Result<(), FinishTemplateInitializationErrorResult> {
        self.metadata_storage
            .execute_under_lock(template_hash, |template_metadata| {
                let Some(template_metadata) = template_metadata else {
                    return Err(FinishTemplateInitializationErrorResult::TemplateWasNotFound);
                };

                match template_metadata.initialization_state {
                    TemplateInitializationState::Done => Ok(()),
                    TemplateInitializationState::Failed => {
                        Err(FinishTemplateInitializationErrorResult::InitializationIsFailed)
                    }
                    TemplateInitializationState::InProgress { .. } => {
                        template_metadata.initialization_state = TemplateInitializationState::Done;

                        for _ in 0..self.db_pool_configs.min_size {
                            let test_db = TestDbMetadata {
                                id: template_metadata.next_test_db_id(),
                                state: TestDbState::Creating {},
                            };

                            PgTempestCore::start_test_db_creation_in_background(
                                self.clone(),
                                template_hash,
                                test_db.id,
                            );

                            template_metadata.test_dbs.push(test_db);
                        }

                        Ok(())
                    }
                }
            })
            .await
    }
}
