use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::{
    features::templates::TemplatesFeature,
    models::{
        template_database::TemplateInitializationState, value_types::template_hash::TemplateHash,
    },
};

pub struct ExtendTemplateInitializationOkResult {
    pub new_initialization_deadline: DateTime<Utc>,
}

pub enum ExtendTemplateInitializationErrorResult {
    TemplateWasNotFound,
    TemplateIsInitialized,
    TemplateInitializationWasFailed,
}

impl TemplatesFeature {
    pub async fn extend_template_initialization(
        &self,
        template_hash: TemplateHash,
        additional_time: Duration,
    ) -> Result<ExtendTemplateInitializationOkResult, ExtendTemplateInitializationErrorResult> {
        self.state_manager
            .execute_under_lock(template_hash, |state_shard| {
                let Some(state_shard) = state_shard else {
                    return Err(ExtendTemplateInitializationErrorResult::TemplateWasNotFound);
                };

                let initialization_state = &mut state_shard.template_database.initialization_state;

                match initialization_state {
                    TemplateInitializationState::Done => {
                        Err(ExtendTemplateInitializationErrorResult::TemplateIsInitialized)
                    }
                    TemplateInitializationState::Failed => Err(
                        ExtendTemplateInitializationErrorResult::TemplateInitializationWasFailed,
                    ),
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
