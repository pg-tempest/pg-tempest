use std::sync::Arc;

use tracing::{debug, info, instrument, warn};

use crate::{
    PgTempestCore,
    metadata::template_metadata::{
        TemplateAwaitingResult, TemplateInitializationState, TestDbMetadata, TestDbState,
    },
    models::value_types::template_hash::TemplateHash,
};

pub enum FinishTemplateInitializationErrorResult {
    TemplateWasNotFound,
    InitializationIsFailed,
    InitializationIsNotStarted,
}

impl PgTempestCore {
    #[instrument(skip_all)]
    pub async fn finish_template_initialization(
        self: Arc<Self>,
        template_hash: TemplateHash,
    ) -> Result<(), FinishTemplateInitializationErrorResult> {
        self.metadata_storage
            .execute_under_lock(template_hash, |template| {
                let Some(template) = template else {
                    warn!("Template {template_hash} was not found");
                    return Err(FinishTemplateInitializationErrorResult::TemplateWasNotFound);
                };

                match template.initialization_state {
                    TemplateInitializationState::Finished => {
                        debug!("Template {template_hash} initialization is already finished");
                        Ok(())
                    }
                    TemplateInitializationState::Failed => {
                        warn!("Template {template_hash} initialization is failed");
                        Err(FinishTemplateInitializationErrorResult::InitializationIsFailed)
                    }
                    TemplateInitializationState::Created
                    | TemplateInitializationState::Creating => {
                        warn!("Template {template_hash} initialization is not started");
                        Err(FinishTemplateInitializationErrorResult::InitializationIsNotStarted)
                    }
                    TemplateInitializationState::InProgress { .. } => {
                        template.initialization_state = TemplateInitializationState::Finished;

                        while let Some(awaiter) = template.template_awaiters.pop_front() {
                            let _ = awaiter
                                .result_sender
                                .send(TemplateAwaitingResult::InitializationIsFinished);
                        }

                        for _ in 0..self.db_pool_configs.min_size {
                            let test_db = TestDbMetadata {
                                id: template.next_test_db_id(),
                                state: TestDbState::Creating {},
                            };

                            tokio::spawn(self.clone().recreate_test_db(template_hash, test_db.id));

                            template.test_dbs.push(test_db);
                        }

                        info!("Template {template_hash} initialization was finished");
                        Ok(())
                    }
                }
            })
            .await
    }
}
