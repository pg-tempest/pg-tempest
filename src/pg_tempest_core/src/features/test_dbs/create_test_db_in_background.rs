use std::sync::Arc;

use crate::{
    PgTempestCore,
    db_queries::recreate_test_db::recreate_test_db,
    metadata::template_metadata::{TestDbState, TestDbUsage},
    models::value_types::{
        template_db_name::TemplateDbName, template_hash::TemplateHash, test_db_id::TestDbId,
        test_db_name::TestDbName,
    },
};
use anyhow::{anyhow, bail};
use tracing::{error, info};

impl PgTempestCore {
    pub fn start_test_db_creation_in_background(
        self: Arc<Self>,
        template_hash: TemplateHash,
        test_db_id: TestDbId,
    ) {
        tokio::spawn(async move {
            let test_db_name = TestDbName::new(template_hash, test_db_id);

            let db_creation_result = recreate_test_db(
                &self.dbms_connections_pool,
                &test_db_name,
                &TemplateDbName::new(template_hash),
            )
            .await;

            let result = self
                .metadata_storage
                .execute_under_lock(template_hash, |template| {
                    let Some(template) = template else {
                        bail!("Template {template_hash} was not found");
                    };

                    let test_db = template
                        .test_dbs
                        .iter_mut()
                        .find(|x| x.id == test_db_id)
                        .ok_or(anyhow!("Test db {test_db_id} was not found"))?;

                    if let Err(_) = db_creation_result {
                        test_db.state = TestDbState::Corrupted;
                        bail!("Failed to create {test_db_name}");
                    }

                    if let Some(test_db_waiter) = template.test_db_waiters.pop_front() {
                        let usage_deadline = self.clock.now() + test_db_waiter.usage_duration;
                        let usage = TestDbUsage {
                            test_db_id,
                            deadline: usage_deadline,
                        };

                        if let Ok(_) = test_db_waiter.readines_sender.send(usage) {
                            test_db.state = TestDbState::InUse { usage_deadline };
                            return Ok(());
                        }
                    }

                    test_db.state = TestDbState::Ready;

                    info!("Db {test_db_name} is ready");

                    Ok(())
                })
                .await;

            if let Err(err) = result {
                error!("Db {test_db_name} background creation failed: {err:?}");
            }
        });
    }
}
