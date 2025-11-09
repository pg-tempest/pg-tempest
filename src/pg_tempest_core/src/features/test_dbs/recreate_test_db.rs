use std::sync::Arc;

use crate::{
    PgTempestCore,
    metadata::template_metadata::{TestDbState, TestDbUsage},
    models::value_types::{
        template_db_name::TemplateDbName, template_hash::TemplateHash, test_db_id::TestDbId,
        test_db_name::TestDbName,
    },
    pg_client_extensions::PgClientExtensions,
};
use anyhow::{anyhow, bail};
use tracing::{debug, error, instrument};

impl PgTempestCore {
    #[instrument(skip_all)]
    pub async fn recreate_test_db(
        self: Arc<Self>,
        template_hash: TemplateHash,
        test_db_id: TestDbId,
    ) {
        let test_db_name = TestDbName::new(template_hash, test_db_id);

        let db_creation_result = self
            .pg_client
            .recreate_test_db(&test_db_name, &TemplateDbName::new(template_hash))
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

                while let Some(test_db_awaiter) = template.test_db_awaiters.pop_front() {
                    let usage_deadline = self.clock.now() + test_db_awaiter.usage_duration;
                    let usage = TestDbUsage {
                        test_db_id,
                        deadline: usage_deadline,
                    };

                    if let Ok(_) = test_db_awaiter.readines_sender.send(usage) {
                        test_db.state = TestDbState::InUse { usage_deadline };
                        return Ok(());
                    }
                }

                test_db.state = TestDbState::Ready;

                debug!("Test db {template_hash} {test_db_id} was returned to pool");

                Ok(())
            })
            .await;

        if let Err(err) = result {
            error!("Failed to recreate test db {template_hash} {test_db_id}: {err}");
        }
    }
}
