use crate::{
    features::templates::TemplatesFeature,
    metadata::template_metadata::TemplateInitializationState,
    models::value_types::template_hash::TemplateHash,
};

pub enum FinishTemplateInitializationErrorResult {
    TemplateWasNotFound,
    InitializationIsFailed,
}

impl TemplatesFeature {
    pub async fn finish_template_initialization(
        &self,
        template_hash: TemplateHash,
    ) -> Result<(), FinishTemplateInitializationErrorResult> {
        self.metadata_storage
            .execute_under_lock(template_hash, |template_metadata| match template_metadata {
                None => Err(FinishTemplateInitializationErrorResult::TemplateWasNotFound),
                Some(template_metadata) => match template_metadata.initialization_state {
                    TemplateInitializationState::Done => Ok(()),
                    TemplateInitializationState::Failed => {
                        Err(FinishTemplateInitializationErrorResult::InitializationIsFailed)
                    }
                    TemplateInitializationState::InProgress { .. } => {
                        template_metadata.initialization_state = TemplateInitializationState::Done;
                        Ok(())
                    }
                },
            })
            .await
    }
}
