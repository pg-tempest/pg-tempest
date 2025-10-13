use crate::{
    features::templates::TemplatesFeature,
    models::{
        template_database::TemplateInitializationState, value_types::template_hash::TemplateHash,
    },
};

pub enum MarkTemplateInitializationAsFailedErrorResult {
    TemplateWasNotFound,
    TemplateIsInitialized,
}

impl TemplatesFeature {
    pub async fn mark_template_initialization_as_failed(
        &self,
        template_hash: TemplateHash,
    ) -> Result<(), MarkTemplateInitializationAsFailedErrorResult> {
        self.state_manager
            .execute_under_lock(template_hash, |state_shard| match state_shard {
                None => Err(MarkTemplateInitializationAsFailedErrorResult::TemplateWasNotFound),
                Some(state_shard) => {
                    let initialization_state =
                        &mut state_shard.template_database.initialization_state;

                    if let TemplateInitializationState::Done = initialization_state {
                        return Err(
                            MarkTemplateInitializationAsFailedErrorResult::TemplateIsInitialized,
                        );
                    }

                    *initialization_state = TemplateInitializationState::Failed;

                    Ok(())
                }
            })
            .await
    }
}
