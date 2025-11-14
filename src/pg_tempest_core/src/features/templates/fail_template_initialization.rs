use derive_more::Display;
use std::sync::Arc;
use thiserror::Error;
use tracing::{debug, info, instrument, warn};

use crate::{
    PgTempestCore,
    metadata::template_metadata::{TemplateAwaitingResult, TemplateInitializationState},
    models::value_types::template_hash::TemplateHash,
};

#[derive(Error, Debug, Display)]
#[display("{self:?}")]
pub enum FailTemplateInitializationErrorResult {
    TemplateWasNotFound,
    InitializationIsFinished,
    InitializationIsNotStarted,
}

impl PgTempestCore {
    #[instrument(skip_all)]
    pub async fn fail_template_initialization(
        self: Arc<Self>,
        template_hash: TemplateHash,
    ) -> Result<(), FailTemplateInitializationErrorResult> {
        self.metadata_storage
            .execute_under_lock(template_hash, |template| {
                let Some(template) = template else {
                    warn!("Template {template_hash} was not found");
                    return Err(FailTemplateInitializationErrorResult::TemplateWasNotFound);
                };

                let initialization_state = &mut template.initialization_state;

                match initialization_state {
                    TemplateInitializationState::Finished => {
                        warn!("Template {template_hash} initialization is finished");
                        return Err(
                            FailTemplateInitializationErrorResult::InitializationIsFinished,
                        );
                    }
                    TemplateInitializationState::Failed => {
                        debug!("Template {template_hash} initialization is already failed");
                        return Ok(());
                    }
                    TemplateInitializationState::Created => {
                        warn!("Template {template_hash} initialization is not started");
                        return Err(
                            FailTemplateInitializationErrorResult::InitializationIsNotStarted,
                        );
                    }
                    TemplateInitializationState::Creating
                    | TemplateInitializationState::InProgress { .. } => {
                        info!("Template {template_hash} initialization was failed");

                        while let Some(awaiter) = template.template_awaiters.pop_front() {
                            let _ = awaiter
                                .result_sender
                                .send(TemplateAwaitingResult::InitializationIsFailed);
                        }

                        *initialization_state = TemplateInitializationState::Failed;

                        return Ok(());
                    }
                };
            })
            .await
    }
}
