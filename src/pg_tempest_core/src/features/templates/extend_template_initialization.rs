use std::time::Duration;

use chrono::{DateTime, Utc};

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
    InitializationIsFailed,
}

impl PgTempestCore {
    pub async fn extend_template_initialization(
        &self,
        template_hash: TemplateHash,
        additional_time: Duration,
    ) -> Result<ExtendTemplateInitializationOkResult, ExtendTemplateInitializationErrorResult> {
        self.metadata_storage
            .execute_under_lock(template_hash, |template_metadata| {
                let Some(template_metadata) = template_metadata else {
                    return Err(ExtendTemplateInitializationErrorResult::TemplateWasNotFound);
                };

                let initialization_state = &mut template_metadata.initialization_state;

                match initialization_state {
                    TemplateInitializationState::Done => {
                        Err(ExtendTemplateInitializationErrorResult::InitializationIsFinished)
                    }
                    TemplateInitializationState::Failed => {
                        Err(ExtendTemplateInitializationErrorResult::InitializationIsFailed)
                    }
                    TemplateInitializationState::InProgress {
                        initialization_deadline,
                    } => {
                        *initialization_deadline = *initialization_deadline + additional_time;
                        Ok(ExtendTemplateInitializationOkResult {
                            new_initialization_deadline: *initialization_deadline,
                        })
                    }
                }
            })
            .await
    }
}
