use tracing::{instrument, warn};

use crate::{
    PgTempestCore, metadata::template_metadata::TemplateInitializationState,
    models::value_types::template_hash::TemplateHash,
};

pub enum FailTemplateInitializationErrorResult {
    TemplateWasNotFound,
    TemplateIsInitialized,
}

impl PgTempestCore {
    #[instrument(skip_all)]
    pub async fn fail_template_initialization(
        &self,
        template_hash: TemplateHash,
    ) -> Result<(), FailTemplateInitializationErrorResult> {
        self.metadata_storage
            .execute_under_lock(template_hash, |template_metadata| {
                let Some(template_metadata) = template_metadata else {
                    warn!("Template {template_hash} was not found");
                    return Err(FailTemplateInitializationErrorResult::TemplateWasNotFound);
                };

                let initialization_state = &mut template_metadata.initialization_state;

                if let TemplateInitializationState::Done = initialization_state {
                    warn!("Template {template_hash} initialization is already finished");
                    return Err(FailTemplateInitializationErrorResult::TemplateIsInitialized);
                }

                *initialization_state = TemplateInitializationState::Failed;

                Ok(())
            })
            .await
    }
}
