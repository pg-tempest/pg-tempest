use crate::{
    features::templates::TemplatesFeature,
    models::{
        template_database::TemplateInitializationState, value_types::template_hash::TemplateHash,
    },
};

pub enum FinishTemplateInitializationErrorResult {
    TemplateNotFound,
    TemplateInitializationWasFailed,
}

impl TemplatesFeature {
    pub async fn finish_template_initialization(
        &self,
        template_hash: TemplateHash,
    ) -> Result<(), FinishTemplateInitializationErrorResult> {
        self.state_manager
            .execute_under_lock(template_hash, |state_shard| match state_shard {
                None => Err(FinishTemplateInitializationErrorResult::TemplateNotFound),
                Some(state_shard) => match state_shard.template_database.initialization_state {
                    TemplateInitializationState::Done => Ok(()),
                    TemplateInitializationState::Failed => Err(
                        FinishTemplateInitializationErrorResult::TemplateInitializationWasFailed,
                    ),
                    TemplateInitializationState::InProgress { .. } => {
                        state_shard.template_database.initialization_state =
                            TemplateInitializationState::Done;
                        Ok(())
                    }
                },
            })
            .await
    }
}
