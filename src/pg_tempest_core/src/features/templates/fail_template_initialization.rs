use crate::utils::option_ext::OptionExt;
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
#[display("FailTemplateInitializationError::{self:?}")]
pub enum FailTemplateInitializationError {
    TemplateWasNotFound { template_hash: TemplateHash },
    InitializationIsFinished,
    InitializationIsNotStarted,
}

impl PgTempestCore {
    #[instrument(skip_all)]
    pub async fn fail_template_initialization(
        self: Arc<Self>,
        template_hash: TemplateHash,
        reason: Option<Arc<str>>,
    ) -> Result<(), FailTemplateInitializationError> {
        self.metadata_storage
            .execute_under_lock(template_hash, |template| {
                let Some(template) = template else {
                    warn!("Template {template_hash} was not found");
                    return Err(FailTemplateInitializationError::TemplateWasNotFound { template_hash });
                };

                let initialization_state = &mut template.initialization_state;

                match initialization_state {
                    TemplateInitializationState::Finished => {
                        warn!("Template {template_hash} initialization is finished");
                        return Err(
                            FailTemplateInitializationError::InitializationIsFinished,
                        );
                    }
                    TemplateInitializationState::Failed { reason } => {
                        debug!(
                            "Template {template_hash} initialization is already failed with reason {}",
                            reason.as_format_arg()
                        );

                        return Ok(());
                    }
                    TemplateInitializationState::Created => {
                        warn!("Template {template_hash} initialization is not started");
                        return Err(
                            FailTemplateInitializationError::InitializationIsNotStarted,
                        );
                    }
                    TemplateInitializationState::Creating
                    | TemplateInitializationState::InProgress { .. } => {
                        info!(
                            "Template {template_hash} initialization was failed with reason {}",
                            reason.as_format_arg()
                        );

                        let reason = reason.map(|x| x.into());

                        while let Some(awaiter) = template.template_awaiters.pop_front() {
                            let _ = awaiter.result_sender.send(
                                TemplateAwaitingResult::InitializationIsFailed {
                                    reason: reason.clone(),
                                },
                            );
                        }

                        *initialization_state = TemplateInitializationState::Failed { reason };

                        return Ok(());
                    }
                };
            })
            .await
    }
}
