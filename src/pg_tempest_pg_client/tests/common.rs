use pg_tempest_core::configs::dbms_configs::{DbmsConfigs, InnerDbmsConfigs, OuterDbmsConfigs};
use pg_tempest_pg_client::pg_client_impl::PgClientImpl;
use std::sync::Arc;
use testcontainers::ContainerAsync;
use testcontainers_modules::postgres::Postgres;

const TEST_PG_DATABASE: &str = "postgres";
const TEST_PG_USER: &str = "postgres";
const TEST_PG_PASSWORD: &str = "postgres";

pub async fn create_pg_client(postgresql_container: &ContainerAsync<Postgres>) -> PgClientImpl {
    let host = postgresql_container.get_host().await.unwrap();
    let port = postgresql_container.get_host_port_ipv4(5432).await.unwrap();

    let configs = Arc::new(DbmsConfigs {
        database: TEST_PG_DATABASE.into(),
        inner: InnerDbmsConfigs {
            host: host.to_string().into(),
            port,
        },
        password: TEST_PG_PASSWORD.into(),
        user: TEST_PG_USER.into(),
        outer: OuterDbmsConfigs { host: None, port: None },
    });

    PgClientImpl::new(configs)
        .await
        .expect("failed to create PgClient")
}
