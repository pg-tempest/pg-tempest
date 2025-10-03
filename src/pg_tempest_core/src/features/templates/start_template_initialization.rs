use crate::features::command::{Command, CommandHandler};
use crate::models::template_database::{TemplateDatabase, TemplateHash};
use crate::template_repository::TemplateRepository;
use crate::utils::clock::Clock;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use std::error::Error;
use std::sync::Arc;

pub struct StartTemplateInitializationCommand {
    pub hash: TemplateHash,
    pub idempotent: bool,
    pub initialization_duration: Duration,
}

pub enum OkResult {
    Started,
    InProgress,
    AlreadyInitialized { template_database: TemplateDatabase },
}

impl Command for StartTemplateInitializationCommand {
    type Response = Result<OkResult, Box<dyn Error>>;
}

pub struct Handler {
    template_repository: Arc<TemplateRepository>,
    clock: Arc<dyn Clock>,
}

#[async_trait]
impl CommandHandler<StartTemplateInitializationCommand> for Handler {
    async fn handle(
        &self,
        command: StartTemplateInitializationCommand,
    ) -> Result<OkResult, Box<dyn Error>> {
        let template_database = TemplateDatabase {
            hash: command.hash,
            has_idempotent_initialization: command.idempotent,
            initialization_attempts: 1,
            initialization_deadline_at: Utc::now() + command.initialization_duration,
            initialized: false,
        };

        let template_database = self
            .template_repository
            .get_or_insert(template_database)
            .await?;

        if template_database.initialized {
            return Ok(OkResult::AlreadyInitialized { template_database });
        }

        if template_database.initialization_deadline_at < self.clock.now() {
            template_database
        }

        // TODO: Find existed template
        //  If template exists and initialized, then return template.
        //  If template exists and not initialized, then return InProgress.
        //  If template exists and marked as failed,
        //      then create new template and start initialization.
        //  If template exists and template has idempotent initialization
        //  and initialization reached timeout, then continue initialization
        //  If template exists and template has not idempotent initialization
        //  and

        todo!()
    }
}
