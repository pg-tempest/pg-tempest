use std::{error::Error, sync::Arc};

use pg_tempest_core::PgTempestCore;
use pg_tempest_server::Server;

use crate::{configs::build_app_configs, logging::setup_logging};

mod configs;
pub mod logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let configs = build_app_configs()?;

    setup_logging(configs.logging.clone())?;

    let tempest_core = PgTempestCore::new(configs.dbms.clone(), configs.db_pool.clone()).await?;

    let tempest_core = Arc::new(tempest_core);

    let server = Server::new(tempest_core.clone(), configs.server.clone());

    tempest_core.start_test_db_creation_retries_in_background();
    server.start().await?;

    Ok(())
}
