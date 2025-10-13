use crate::db_queries::create_db::create_database;
use crate::db_queries::drop_template_db::drop_template_db;
use crate::db_queries::get_dbs::get_dbs;
use crate::features::templates::TemplatesFeature;
use crate::models::db_connection_options::DbConnectionOptions;
use crate::models::template_database::{TemplateDb, TemplateInitializationState};
use crate::models::value_types::template_db_name::TemplateDbName;
use crate::models::value_types::template_hash::TemplateHash;
use crate::state_manager::StateShard;
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
                let db_creation_result = recreate_template_db(self, &template_database_name).await;

                match db_creation_result {
                    Ok(_) => Ok(StartTemplateInitializationOkResult::Started {
                        database_connection_options: DbConnectionOptions {
                            host: self.configs.dbms.host.clone(),
                            port: self.configs.dbms.port,
                            username: self.configs.dbms.user.clone(),
                            password: self.configs.dbms.password.clone(),
                            database: template_database_name.into(),
                        },
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
        .state_manager
        .execute_under_lock(template_hash, |state_shard| match state_shard {
            None => {
                let initialization_deadline = now + initialization_duration;

                let template_database = TemplateDb {
                    hash: template_hash,
                    initialization_state: TemplateInitializationState::InProgress {
                        initialization_deadline,
                    },
                };
                *state_shard = Some(StateShard {
                    template_database,
                    test_database_usages: Vec::default(),
                    test_databases: Vec::default(),
                });

                return DesitionResult::RestartInitialization {
                    template_database_name: TemplateDbName::new(template_hash),
                    initialization_deadline,
                };
            }
            Some(state_shard) => {
                let template_database = &mut state_shard.template_database;

                match template_database.initialization_state {
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

                        template_database.initialization_state =
                            TemplateInitializationState::InProgress {
                                initialization_deadline,
                            };

                        DesitionResult::RestartInitialization {
                            template_database_name,
                            initialization_deadline,
                        }
                    }
                }
            }
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

async fn recreate_template_db(
    feature: &TemplatesFeature,
    template_database_name: &TemplateDbName,
) -> anyhow::Result<()> {
    let databases = get_dbs(&feature.dbms_connections_pool).await?;
    let template_db_exists = databases
        .iter()
        .find(|x| x.name == *template_database_name.as_ref())
        .is_some();

    if template_db_exists {
        drop_template_db(
            &feature.dbms_connections_pool,
            template_database_name.as_ref(),
        )
        .await?;
    }

    create_database(
        &feature.dbms_connections_pool,
        &template_database_name.as_ref(),
        true,
    )
    .await?;

    Ok(())
}

async fn mark_as_failed(feature: &TemplatesFeature, template_hash: TemplateHash) {
    feature
        .state_manager
        .execute_under_lock(template_hash, |state_shard| {
            if let Some(state_shard) = state_shard {
                state_shard.template_database.initialization_state =
                    TemplateInitializationState::Failed
            }
        })
        .await
}
