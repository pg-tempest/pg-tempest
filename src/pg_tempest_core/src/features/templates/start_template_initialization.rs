use std::collections::VecDeque;
use std::time::Duration;

use crate::PgTempestCore;
use crate::metadata::template_metadata::TemplateInitializationState;
use crate::models::db_connection_options::DbConnectionOptions;
use crate::models::value_types::template_db_name::TemplateDbName;
use crate::models::value_types::template_hash::TemplateHash;
use crate::{
    db_queries::recreate_template_db::recreate_template_db,
    metadata::template_metadata::TemplateMetadata,
};
use chrono::{DateTime, Utc};
use tracing::{debug, error, info, instrument};

pub enum StartTemplateInitializationOkResult {
    InitializationWasStarted {
        database_connection_options: DbConnectionOptions,
        initialization_deadline: DateTime<Utc>,
    },
    InitializationIsInProgress {
        initialization_deadline: DateTime<Utc>,
    },
    InitializationIsFinished,
}

impl PgTempestCore {
    #[instrument(skip_all)]
    pub async fn start_template_initialization(
        &self,
        template_hash: TemplateHash,
        initialization_duration: Duration,
    ) -> anyhow::Result<StartTemplateInitializationOkResult> {
        let desition = make_decision(self, template_hash, initialization_duration).await;

        match desition {
            DesitionResult::TemplateInitialized => {
                debug!("Template {template_hash} is already initialized");
                Ok(StartTemplateInitializationOkResult::InitializationIsFinished)
            }
            DesitionResult::InProgress {
                initialization_deadline,
            } => {
                debug!("Template {template_hash} initialization is in progress");

                Ok(
                    StartTemplateInitializationOkResult::InitializationIsInProgress {
                        initialization_deadline,
                    },
                )
            }
            DesitionResult::RestartInitialization {
                template_database_name,
                initialization_deadline,
            } => {
                let db_creation_result =
                    recreate_template_db(&self.dbms_connections_pool, &template_database_name)
                        .await;

                match db_creation_result {
                    Ok(_) => {
                        info!("Template {template_hash} initialization started");

                        Ok(
                            StartTemplateInitializationOkResult::InitializationWasStarted {
                                database_connection_options: DbConnectionOptions::new_outer(
                                    &self.dbms_configs,
                                    template_database_name.into(),
                                ),
                                initialization_deadline,
                            },
                        )
                    }
                    Err(err) => {
                        error!("Template {template_hash} initialization failed: {err}");

                        mark_as_failed(self, template_database_name.into()).await;

                        Err(err)
                    }
                }
            }
        }
    }
}

async fn make_decision(
    tempest_core: &PgTempestCore,
    template_hash: TemplateHash,
    initialization_duration: Duration,
) -> DesitionResult {
    let now = tempest_core.clock.now();

    tempest_core
        .metadata_storage
        .execute_under_lock(template_hash, |template_metadata| match template_metadata {
            None => {
                let initialization_deadline = now + initialization_duration;

                *template_metadata = Some(TemplateMetadata {
                    template_hash,
                    initialization_state: TemplateInitializationState::InProgress {
                        initialization_deadline,
                    },
                    test_dbs: Vec::new(),
                    test_db_waiters: VecDeque::new(),
                    test_db_id_sequence: 0,
                });

                return DesitionResult::RestartInitialization {
                    template_database_name: TemplateDbName::new(template_hash),
                    initialization_deadline,
                };
            }
            Some(template_metadata) => match template_metadata.initialization_state {
                TemplateInitializationState::Done => DesitionResult::TemplateInitialized,
                TemplateInitializationState::InProgress {
                    initialization_deadline,
                } if initialization_deadline > now => DesitionResult::InProgress {
                    initialization_deadline,
                },
                TemplateInitializationState::Failed
                | TemplateInitializationState::InProgress { .. } => {
                    let initialization_deadline = now + initialization_duration;
                    let template_database_name = TemplateDbName::new(template_hash);

                    template_metadata.initialization_state =
                        TemplateInitializationState::InProgress {
                            initialization_deadline,
                        };

                    DesitionResult::RestartInitialization {
                        template_database_name,
                        initialization_deadline,
                    }
                }
            },
        })
        .await
}

enum DesitionResult {
    TemplateInitialized,
    RestartInitialization {
        template_database_name: TemplateDbName,
        initialization_deadline: DateTime<Utc>,
    },
    InProgress {
        initialization_deadline: DateTime<Utc>,
    },
}

async fn mark_as_failed(tempest_core: &PgTempestCore, template_hash: TemplateHash) {
    tempest_core
        .metadata_storage
        .execute_under_lock(template_hash, |template_metadata| {
            if let Some(template_metadata) = template_metadata {
                template_metadata.initialization_state = TemplateInitializationState::Failed
            }
        })
        .await
}
