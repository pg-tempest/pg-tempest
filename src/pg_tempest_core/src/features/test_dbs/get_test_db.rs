use std::time::Duration;

use crate::{
    db_queries::recreate_test_db::recreate_test_db,
    features::test_dbs::TestDbsFeature,
    metadata::template_metadata::{TemplateInitializationState, TestDbMetadata},
    models::{
        db_connection_options::DbConnectionOptions,
        value_types::{
            template_db_name::TemplateDbName, template_hash::TemplateHash, test_db_id::TestDbId,
            test_db_name::TestDbName,
        },
    },
};

pub struct GetTestDbOkResult {
    pub test_db_id: TestDbId,
    pub connection_options: DbConnectionOptions,
}

pub enum GetTestDbErrorResult {
    TemplateWasNotFound,
    TemplateIsNotInitalized,
    Unknown { inner: anyhow::Error },
}

impl TestDbsFeature {
    pub async fn get_test_db(
        &self,
        template_hash: TemplateHash,
        usage_duration: Duration,
    ) -> Result<GetTestDbOkResult, GetTestDbErrorResult> {
        let now = self.clock.now();

        let (template_hash, test_db_id) = self
            .metadata_storage
            .execute_under_lock(template_hash, |template_metadata| {
                let Some(template_metadata) = template_metadata else {
                    return Err(GetTestDbErrorResult::TemplateWasNotFound);
                };

                match template_metadata.initialization_state {
                    TemplateInitializationState::Done => {}
                    _ => return Err(GetTestDbErrorResult::TemplateIsNotInitalized),
                };

                let test_db_metadata = template_metadata.test_dbs.iter_mut().find(|test_db| {
                    test_db.corrupted
                        || test_db
                            .usage_deadline
                            .is_some_and(|deadline| deadline <= now)
                });

                let usage_deadline = now + usage_duration;

                match test_db_metadata {
                    Some(test_db_metadata) => {
                        test_db_metadata.corrupted = false;
                        test_db_metadata.usage_deadline = Some(usage_deadline);

                        Ok((template_hash, test_db_metadata.id))
                    }
                    None => {
                        let next_test_db_id = template_metadata
                            .test_dbs
                            .iter()
                            .map(|x| x.id)
                            .max()
                            .map(|x| x + 1)
                            .unwrap_or(1u16);

                        let test_db_metadata = TestDbMetadata {
                            id: next_test_db_id,
                            corrupted: false,
                            usage_deadline: Some(usage_deadline),
                        };

                        template_metadata.test_dbs.push(test_db_metadata);

                        Ok((template_hash, next_test_db_id))
                    }
                }
            })
            .await?;

        let test_db_name = TestDbName::new(template_hash, test_db_id);
        let db_creation_result = recreate_test_db(
            &self.dbms_connections_pool,
            &test_db_name,
            &TemplateDbName::new(template_hash),
        )
        .await;

        if let Err(error) = db_creation_result {
            self.metadata_storage
                .execute_under_lock(template_hash, |template_metadata| {
                    if let Some(template_metadata) = template_metadata {
                        if let Some(test_db) = template_metadata
                            .test_dbs
                            .iter_mut()
                            .find(|x| x.id == test_db_id)
                        {
                            test_db.corrupted = true;
                            test_db.usage_deadline = None;
                        }
                    }
                })
                .await;

            return Err(GetTestDbErrorResult::Unknown { inner: error });
        }

        Ok(GetTestDbOkResult {
            test_db_id: test_db_id,
            connection_options: DbConnectionOptions::new_outer(
                &self.configs.dbms,
                test_db_name.into(),
            ),
        })
    }
}
