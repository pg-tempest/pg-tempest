use std::sync::Arc;

use tracing::{debug, info, instrument, warn};

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
    #[instrument(skip_all)]
    pub async fn finish_template_initialization(
        self: Arc<PgTempestCore>,
        template_hash: TemplateHash,
    ) -> Result<(), FinishTemplateInitializationErrorResult> {
        self.metadata_storage
            .execute_under_lock(template_hash, |template_metadata| {
                let Some(template_metadata) = template_metadata else {
                    warn!("Template {template_hash} was not found");
                    return Err(FinishTemplateInitializationErrorResult::TemplateWasNotFound);
                };

                match template_metadata.initialization_state {
                    TemplateInitializationState::Done => {
                        debug!("Template {template_hash} initialization is already finished");
                        Ok(())
                    }
                    TemplateInitializationState::Failed => {
                        warn!("Template {template_hash} initialization is already failed");
                        Err(FinishTemplateInitializationErrorResult::InitializationIsFailed)
                    }
                    TemplateInitializationState::InProgress { .. } => {
                        template_metadata.initialization_state = TemplateInitializationState::Done;

                        for _ in 0..self.db_pool_configs.min_size {
                            let test_db = TestDbMetadata {
                                id: template_metadata.next_test_db_id(),
                                state: TestDbState::Creating {},
                            };

                            tokio::spawn(self.clone().recreate_test_db(template_hash, test_db.id));

                            template_metadata.test_dbs.push(test_db);
                        }

                        info!("Template {template_hash} initialization was finished");
                        Ok(())
                    }
                }
            })
            .await
    }
}
