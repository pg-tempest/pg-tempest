use std::sync::Arc;

use pg_tempest_core::PgTempestCore;
use pg_tempest_server::Server;
use tracing::Level;
use tracing_subscriber::{fmt::format::DefaultFields, layer::SubscriberExt};

use crate::configs::build_app_configs;

mod configs;

#[tokio::main]
async fn main() {
    let configs = build_app_configs().unwrap();

    let filter = tracing_subscriber::filter::Targets::new()
        .with_default(Level::DEBUG)
        .with_target("sqlx", Level::WARN);

    let r = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .fmt_fields(DefaultFields::new()),
        )
        .with(filter);

    tracing::subscriber::set_global_default(r).unwrap();

    let tempest_core = PgTempestCore::new(configs.dbms.clone(), configs.db_pool.clone())
        .await
        .unwrap();

    let tempest_core = Arc::new(tempest_core);

    let server = Server::new(tempest_core.clone(), configs.server.clone());

    tempest_core.start_test_db_creation_retries_in_background();
    server.start().await;
}
