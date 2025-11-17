use std::sync::Arc;

use pg_tempest_core::PgTempestCore;
use pg_tempest_core::utils::unexpected_error::UnexpectedError;
use pg_tempest_pg_client::pg_client_impl::PgClientImpl;
use pg_tempest_server::Server;

use crate::{configs::build_app_configs, logging::setup_logging};

mod configs;
pub mod logging;

#[tokio::main]
async fn main() -> Result<(), UnexpectedError> {
    let configs = build_app_configs()?;

    setup_logging(configs.logging.clone())?;

    let pg_client = Arc::new(PgClientImpl::new(configs.dbms.clone()).await?);

    let tempest_core = Arc::new(
        PgTempestCore::new(
            pg_client,
            configs.dbms.clone(),
            configs.db_pool.clone(),
            configs.templates.clone(),
        )
        .await?,
    );

    let server = Server::new(tempest_core.clone(), configs.server.clone());

    tempest_core
        .clone()
        .start_test_db_creation_retries_in_background();
    tempest_core.start_template_initialization_deadline_handling();

    server.start().await?;

    Ok(())
}
