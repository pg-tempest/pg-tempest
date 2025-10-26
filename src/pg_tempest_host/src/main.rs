use std::sync::Arc;

use pg_tempest_core::PgTempestCore;
use pg_tempest_server::Server;

use crate::configs::build_app_configs;

mod configs;

#[tokio::main]
async fn main() {
    let configs = build_app_configs().unwrap();

    let tracing_subscriber = tracing_subscriber::fmt()
        .compact()
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(tracing_subscriber).unwrap();

    let tempest_core = PgTempestCore::new(configs.dbms.clone(), configs.db_pool.clone())
        .await
        .unwrap();

    let tempest_core = Arc::new(tempest_core);

    let server = Server::new(tempest_core.clone(), configs.server.clone());

    tempest_core.start_test_db_creation_retries_in_background();
    server.start().await;
}
