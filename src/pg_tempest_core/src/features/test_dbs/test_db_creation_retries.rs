use std::{sync::Arc, time::Duration};

use tokio::time::sleep;
use tracing::info;

use crate::{
    PgTempestCore, metadata::template_metadata::TestDbState,
    models::value_types::test_db_name::TestDbName,
};

impl PgTempestCore {
    pub fn start_test_db_creation_retries_in_background(self: Arc<Self>) {
        tokio::spawn(async move {
            let retries_delay =
                Duration::from_millis(self.db_pool_configs.creation_retries_delay_in_ms);

            loop {
                sleep(retries_delay).await;

                let template_hashes = self.metadata_storage.get_all_template_hashes().await;

                for template_hash in template_hashes {
                    self.metadata_storage
                        .execute_under_lock(template_hash, |template| {
                            let Some(template) = template else {
                                return;
                            };

                            let now = self.clock.now();

                            for test_db in template.test_dbs.iter_mut() {
                                match test_db.state {
                                    TestDbState::Corrupted => {
                                        info!(
                                            "Retring to recreate {}",
                                            TestDbName::new(template_hash, test_db.id)
                                        );
                                    }
                                    TestDbState::InUse { usage_deadline }
                                        if usage_deadline <= now =>
                                    {
                                        info!(
                                            "{} usage deadline is now. Recreating",
                                            TestDbName::new(template_hash, test_db.id)
                                        );
                                    }
                                    _ => continue,
                                }

                                test_db.state = TestDbState::Creating;

                                self.clone().start_test_db_creation_in_background(
                                    template_hash,
                                    test_db.id,
                                );
                            }
                        })
                        .await;
                }
            }
        });
    }
}
