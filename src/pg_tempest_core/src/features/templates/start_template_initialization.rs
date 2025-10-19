use crate::features::templates::TemplatesFeature;
use crate::metadata::template_metadata::TemplateInitializationState;
use crate::models::db_connection_options::DbConnectionOptions;
use crate::models::value_types::template_db_name::TemplateDbName;
use crate::models::value_types::template_hash::TemplateHash;
use crate::{
    db_queries::recreate_template_db::recreate_template_db,
    metadata::template_metadata::TemplateMetadata,
};
use chrono::{DateTime, Duration, Utc};

pub enum StartTemplateInitializationOkResult {
    Started {
        database_connection_options: DbConnectionOptions,
        initialization_deadline: DateTime<Utc>,
    },
    InProgress {
        initialization_deadline: DateTime<Utc>,
    },
    Initialized,
}

impl TemplatesFeature {
    pub async fn start_template_initialization(
        &self,
        hash: TemplateHash,
        initialization_duration: Duration,
    ) -> anyhow::Result<StartTemplateInitializationOkResult> {
        let desition = make_decision(self, hash, initialization_duration).await;

        match desition {
            DesitionResult::TemplateInitialized => {
                Ok(StartTemplateInitializationOkResult::Initialized)
            }
            DesitionResult::InProgress {
                initialization_deadline,
            } => Ok(StartTemplateInitializationOkResult::InProgress {
                initialization_deadline,
            }),
            DesitionResult::RestartInitialization {
                template_database_name,
                initialization_deadline,
            } => {
                let db_creation_result =
                    recreate_template_db(&self.dbms_connections_pool, &template_database_name)
                        .await;

                match db_creation_result {
                    Ok(_) => Ok(StartTemplateInitializationOkResult::Started {
                        database_connection_options: DbConnectionOptions::new_outer(
                            &self.configs.dbms,
                            template_database_name.into(),
                        ),
                        initialization_deadline,
                    }),
                    Err(err) => {
                        mark_as_failed(self, template_database_name.into()).await;

                        Err(err)
                    }
                }
            }
        }
    }
}

async fn make_decision(
    feature: &TemplatesFeature,
    template_hash: TemplateHash,
    initialization_duration: Duration,
) -> DesitionResult {
    let now = feature.clock.now();

    feature
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

async fn mark_as_failed(feature: &TemplatesFeature, template_hash: TemplateHash) {
    feature
        .metadata_storage
        .execute_under_lock(template_hash, |template_metadata| {
            if let Some(template_metadata) = template_metadata {
                template_metadata.initialization_state = TemplateInitializationState::Failed
            }
        })
        .await
}
