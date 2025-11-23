use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use tracing::{info, instrument, warn};

use crate::{
    PgTempestCore, metadata::template_metadata::TemplateInitializationState,
    models::value_types::template_hash::TemplateHash,
};

pub struct ExtendTemplateInitializationOkResult {
    pub new_initialization_deadline: DateTime<Utc>,
}

pub enum ExtendTemplateInitializationErrorResult {
    TemplateWasNotFound,
    InitializationIsFinished,
    InitializationIsFailed { reason: Option<Arc<str>> },
    InitializationIsNotStarted,
}

impl PgTempestCore {
    #[instrument(skip_all)]
    pub async fn extend_template_initialization(
        &self,
        template_hash: TemplateHash,
        additional_time: Duration,
    ) -> Result<ExtendTemplateInitializationOkResult, ExtendTemplateInitializationErrorResult> {
        self.metadata_storage
            .execute_under_lock(template_hash, |template_metadata| {
                let Some(template_metadata) = template_metadata else {
                    warn!("Template {template_hash} was not found");
                    return Err(ExtendTemplateInitializationErrorResult::TemplateWasNotFound);
                };

                let initialization_state = &mut template_metadata.initialization_state;

                match initialization_state {
                    TemplateInitializationState::Finished => {
                        warn!("Template {template_hash} initialization is finished");
                        Err(ExtendTemplateInitializationErrorResult::InitializationIsFinished)
                    }
                    TemplateInitializationState::Failed { reason } => {
                        warn!("Template {template_hash} initialization is failed");
                        Err(
                            ExtendTemplateInitializationErrorResult::InitializationIsFailed {
                                reason: reason.clone(),
                            },
                        )
                    }
                    TemplateInitializationState::Created
                    | TemplateInitializationState::Creating => {
                        warn!("Template {template_hash} initialization is not started");
                        Err(ExtendTemplateInitializationErrorResult::InitializationIsNotStarted)
                    }
                    TemplateInitializationState::InProgress {
                        initialization_deadline,
                    } => {
                        *initialization_deadline = *initialization_deadline + additional_time;
                        info!(
                            "Template {template_hash} initialization is extended by {}",
                            additional_time.as_millis()
                        );
                        Ok(ExtendTemplateInitializationOkResult {
                            new_initialization_deadline: *initialization_deadline,
                        })
                    }
                }
            })
            .await
    }
}
