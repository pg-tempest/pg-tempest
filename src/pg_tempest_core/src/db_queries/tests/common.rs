use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::time::Duration;
use testcontainers::ContainerAsync;
use testcontainers_modules::postgres::Postgres;

const TEST_PG_USER: &str = "postgres";
const TEST_PG_PASSWORD: &str = "postgres";

pub async fn create_pg_pool(postgresql_container: &ContainerAsync<Postgres>) -> PgPool {
    let host = postgresql_container.get_host().await.unwrap();
    let port = postgresql_container.get_host_port_ipv4(5432).await.unwrap();

    let pg_connect_options = PgConnectOptions::new_without_pgpass()
        .host(host.to_string().as_str())
        .port(port)
        .username(TEST_PG_USER)
        .password(TEST_PG_PASSWORD);

    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(pg_connect_options)
        .await
        .expect("cannot connect to database")
}
