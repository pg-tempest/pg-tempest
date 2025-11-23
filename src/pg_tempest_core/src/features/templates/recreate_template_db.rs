use std::sync::Arc;
use tracing::{debug, error};

use crate::models::value_types::pg_identifier::PgIdentifier;
use crate::pg_client_extensions::RecreateTemplateDbError;
use crate::utils::errors::{ArcDynError, BoxDynError};
use crate::{
    PgTempestCore,
    metadata::template_metadata::{TemplateAwaitingResult, TemplateInitializationState},
    models::value_types::{template_db_name::TemplateDbName, template_hash::TemplateHash},
    pg_client_extensions::PgClientExtensions,
};

impl PgTempestCore {
    pub async fn recreate_template_db(
        self: Arc<PgTempestCore>,
        template_hash: TemplateHash,
        parent_template_db_name: Option<PgIdentifier>,
    ) {
        let template_db_name = TemplateDbName::new(template_hash);
        let parent_template_db_name =
            parent_template_db_name.or(self.templates_configs.parent_template_db_name.clone());

        let db_creation_result = self
            .pg_client
            .recreate_template_db(template_db_name.clone().into(), parent_template_db_name)
            .await;

        match db_creation_result {
            Ok(_) => {
                debug!("{template_db_name} was created");
                send_template_awaiting_results(&self, template_hash).await
            }
            Err(RecreateTemplateDbError::ParentTemplateDbDoesNotExist {
                parent_template_db_name,
            }) => {
                let fail_reason =
                    format!("Parent template db {parent_template_db_name} was not found").into();

                let fail_result = self
                    .fail_template_initialization(template_hash, Some(fail_reason))
                    .await;

                if let Err(err) = fail_result {
                    error!("{err}");
                };
            }
            Err(err) => {
                send_template_awaiting_unexpected_error(&self, template_hash, err.into()).await;
            }
        };
    }
}

async fn send_template_awaiting_results(
    pg_tempest_core: &PgTempestCore,
    template_hash: TemplateHash,
) {
    pg_tempest_core
        .metadata_storage
        .execute_under_lock(template_hash, |template| {
            let Some(template) = template else {
                error!("Template {template_hash} was not found after db was created");
                return;
            };

            while let Some(template_awaiter) = template.template_awaiters.pop_front() {
                let initialization_deadline =
                    pg_tempest_core.clock.now() + template_awaiter.initialization_duration;

                let awaiting_result = TemplateAwaitingResult::InitializationIsStarted {
                    initialization_deadline,
                };

                if let Ok(_) = template_awaiter.result_sender.send(awaiting_result) {
                    template.initialization_state = TemplateInitializationState::InProgress {
                        initialization_deadline,
                    };

                    return;
                }
            }

            template.initialization_state = TemplateInitializationState::Created;
        })
        .await
}

async fn send_template_awaiting_unexpected_error(
    pg_tempest_core: &PgTempestCore,
    template_hash: TemplateHash,
    error: BoxDynError,
) {
    pg_tempest_core
        .metadata_storage
        .execute_under_lock(template_hash, |template| {
            let Some(template) = template else {
                error!("Template {template_hash} was not found after db was created");
                return;
            };

            let error: ArcDynError = error.into();

            while let Some(template_awaiter) = template.template_awaiters.pop_front() {
                let awaiting_result = TemplateAwaitingResult::UnexpectedError(error.clone());

                let _ = template_awaiter.result_sender.send(awaiting_result);
            }

            template.initialization_state = TemplateInitializationState::Failed {
                reason: Some(error.to_string().into()),
            };
        })
        .await
}
