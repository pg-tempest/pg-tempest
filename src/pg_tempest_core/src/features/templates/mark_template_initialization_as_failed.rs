use crate::{
    features::templates::TemplatesFeature,
    metadata::template_metadata::TemplateInitializationState,
    models::value_types::template_hash::TemplateHash,
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
        self.metadata_storage
            .execute_under_lock(template_hash, |template_metadata| {
                let Some(template_metadata) = template_metadata else {
                    return Err(MarkTemplateInitializationAsFailedErrorResult::TemplateWasNotFound);
                };

                let initialization_state = &mut template_metadata.initialization_state;

                if let TemplateInitializationState::Done = initialization_state {
                    return Err(
                        MarkTemplateInitializationAsFailedErrorResult::TemplateIsInitialized,
                    );
                }

                *initialization_state = TemplateInitializationState::Failed;

                Ok(())
            })
            .await
    }
}
