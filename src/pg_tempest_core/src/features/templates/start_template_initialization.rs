use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

use crate::PgTempestCore;
use crate::metadata::template_metadata::TemplateAwaiter;
use crate::metadata::template_metadata::TemplateAwaitingResult;
use crate::metadata::template_metadata::TemplateInitializationState;
use crate::metadata::template_metadata::TemplateMetadata;
use crate::models::db_connection_options::DbConnectionOptions;
use crate::models::value_types::pg_identifier::PgIdentifier;
use crate::models::value_types::template_db_name::TemplateDbName;
use crate::models::value_types::template_hash::TemplateHash;
use crate::utils::errors::{BoxDynError, ErrorArcDynError};
use chrono::{DateTime, Utc};
use tokio::select;
use tokio::sync::oneshot;
use tokio::time::sleep;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::instrument;

pub enum StartTemplateInitializationResult {
    InitializationWasStarted {
        database_connection_options: DbConnectionOptions,
        initialization_deadline: DateTime<Utc>,
    },
    InitializationIsInProgress,
    InitializationIsFinished,
    InitializationIsFailed {
        reason: Option<Arc<str>>,
    },
}

impl PgTempestCore {
    #[instrument(skip_all)]
    pub async fn start_template_initialization(
        self: Arc<Self>,
        template_hash: TemplateHash,
        initialization_duration: Duration,
        parent_template_db_name: Option<PgIdentifier>,
    ) -> Result<StartTemplateInitializationResult, BoxDynError> {
        let result_receiver: oneshot::Receiver<TemplateAwaitingResult> = self
            .metadata_storage
            .execute_under_lock(template_hash, |template| {
                let (result_sender, result_receiver) = oneshot::channel::<TemplateAwaitingResult>();

                let Some(template) = template else {
                    let mut template_awaiters = VecDeque::new();
                    template_awaiters.push_back(TemplateAwaiter {
                        initialization_duration,
                        result_sender,
                    });

                    *template = Some(TemplateMetadata {
                        template_hash,
                        initialization_state: TemplateInitializationState::Creating,
                        template_awaiters,
                        test_dbs: Vec::new(),
                        test_db_awaiters: VecDeque::new(),
                        test_db_id_sequence: 0,
                    });

                    tokio::spawn(
                        self.clone()
                            .recreate_template_db(template_hash, parent_template_db_name),
                    );

                    return result_receiver;
                };

                let initialization_state = &mut template.initialization_state;

                match initialization_state {
                    TemplateInitializationState::Created => {
                        let initialization_deadline = self.clock.now() + initialization_duration;

                        if let Ok(_) =
                            result_sender.send(TemplateAwaitingResult::InitializationIsStarted {
                                initialization_deadline,
                            })
                        {
                            *initialization_state = TemplateInitializationState::InProgress {
                                initialization_deadline,
                            };
                        };
                    }
                    TemplateInitializationState::Creating
                    | TemplateInitializationState::InProgress { .. } => {
                        template.template_awaiters.push_back(TemplateAwaiter {
                            initialization_duration,
                            result_sender,
                        });
                    }
                    TemplateInitializationState::Finished => {
                        let _ =
                            result_sender.send(TemplateAwaitingResult::InitializationIsFinished);
                    }
                    TemplateInitializationState::Failed { .. } => {
                        *initialization_state = TemplateInitializationState::Creating;

                        template.template_awaiters.push_back(TemplateAwaiter {
                            initialization_duration,
                            result_sender,
                        });

                        tokio::spawn(
                            self.clone()
                                .recreate_template_db(template_hash, parent_template_db_name),
                        );
                    }
                };

                result_receiver
            })
            .await;

        let long_polling_timeout = Duration::from_millis(
            self.templates_configs
                .initialization
                .long_polling_timeout_ms,
        );

        let awaiting_result = select! {
            _ = sleep(long_polling_timeout) => {
                return Ok(StartTemplateInitializationResult::InitializationIsInProgress)
            }
            awaiting_result = result_receiver => { awaiting_result }
        };

        // TODO: Remove unwrap
        match awaiting_result.unwrap() {
            TemplateAwaitingResult::InitializationIsStarted {
                initialization_deadline,
            } => {
                info!("Template {template_hash} initialization was started");
                let template_db_name = TemplateDbName::new(template_hash);

                Ok(
                    StartTemplateInitializationResult::InitializationWasStarted {
                        database_connection_options: DbConnectionOptions::new_outer(
                            &self.dbms_configs,
                            template_db_name.into(),
                        ),
                        initialization_deadline,
                    },
                )
            }
            TemplateAwaitingResult::InitializationIsInProgress => {
                debug!("Template {template_hash} initialization is already in progress");

                Ok(StartTemplateInitializationResult::InitializationIsInProgress)
            }
            TemplateAwaitingResult::InitializationIsFinished => {
                debug!("Template {template_hash} initialization is already finished");

                Ok(StartTemplateInitializationResult::InitializationIsFinished)
            }
            TemplateAwaitingResult::InitializationIsFailed { reason } => {
                info!("Template {template_hash} initialization was failed");

                Ok(StartTemplateInitializationResult::InitializationIsFailed { reason })
            }
            TemplateAwaitingResult::UnexpectedError(error) => {
                error!("Template db {template_hash} creation was failed: {error}");

                Err(Box::new(ErrorArcDynError(error)))
            }
        }
    }
}
