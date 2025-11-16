use crate::PgTempestCore;
use crate::metadata::template_metadata::TemplateInitializationState;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info};

impl PgTempestCore {
    pub fn start_template_initialization_deadline_handling(self: Arc<Self>) {
        tokio::spawn(async move {
            loop {
                let delay = Duration::from_millis(
                    self.templates_configs
                        .initialization
                        .max_deadline_handling_delay_ms,
                );

                sleep(delay).await;

                let template_hashes = self.metadata_storage.get_all_template_hashes().await;

                for template_hash in template_hashes {
                    self.metadata_storage
                        .execute_under_lock(template_hash, |template| {
                            let Some(template) = template else {
                                error!("Template {template_hash} was not found");
                                return ();
                            };

                            if let TemplateInitializationState::InProgress {
                                initialization_deadline
                            } = &template.initialization_state
                                && *initialization_deadline <= self.clock.now()
                            {
                                info!("Template {template_hash} initialization deadline is now. Failing");
                                tokio::spawn(
                                    self.clone().fail_template_initialization(template_hash),
                                );
                            };
                        })
                        .await
                }
            }
        });
    }
}
