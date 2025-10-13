use std::sync::Arc;

use pg_tempest_core::PgTempestCore;
use pg_tempest_server::Server;

use crate::configs::build_app_configs;

mod configs;

#[tokio::main]
async fn main() {
    let configs = build_app_configs().unwrap();

    let tempest_core = PgTempestCore::new(configs.core.clone()).await.unwrap();
    let tempest_core = Arc::new(tempest_core);

    let server = Server::new(tempest_core, configs.server.clone());

    server.start().await;
}
