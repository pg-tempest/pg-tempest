use std::sync::Arc;

use tracing::{debug, error};

use crate::{
    PgTempestCore,
    metadata::template_metadata::{TemplateAwaitingResult, TemplateInitializationState},
    models::value_types::{template_db_name::TemplateDbName, template_hash::TemplateHash},
    pg_client_extensions::PgClientExtensions,
};

impl PgTempestCore {
    pub async fn recreate_template_db(self: Arc<PgTempestCore>, template_hash: TemplateHash) {
        let template_db_name = TemplateDbName::new(template_hash);

        let db_creation_result = self.pg_client.recreate_template_db(&template_db_name).await;

        if let Err(err) = db_creation_result {
            error!("Failed to create db {template_db_name}: {err}");

            if let Err(err) = self.fail_template_initialization(template_hash).await {
                error!("Failed to fail template {template_hash} initialization: {err}");
            };

            return;
        }

        debug!("Template db {template_hash} was created");

        self.metadata_storage
            .execute_under_lock(template_hash, |template| {
                let Some(template) = template else {
                    error!("Template {template_hash} was not found after db was created");
                    return ();
                };

                while let Some(template_awaiter) = template.template_awaiters.pop_front() {
                    let initialization_deadline =
                        self.clock.now() + template_awaiter.initialization_duration;

                    let awaiting_result = TemplateAwaitingResult::InitializationIsStarted {
                        initialization_deadline,
                    };

                    if let Ok(_) = template_awaiter.result_sender.send(awaiting_result) {
                        template.initialization_state = TemplateInitializationState::InProgress {
                            initialization_deadline,
                        };

                        return ();
                    }
                }

                template.initialization_state = TemplateInitializationState::Created;
            })
            .await;
    }
}
