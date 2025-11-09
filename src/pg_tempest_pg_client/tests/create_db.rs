use pg_tempest_core::{
    models::value_types::pg_identifier::PgIdentifier,
    pg_client::{CreateDatabaseError, PgClient},
};
use testcontainers::runners::AsyncRunner;

mod common;

#[tokio::test]
async fn db_double_creation() {
    let postgresql_container = testcontainers_modules::postgres::Postgres::default()
        .start()
        .await
        .unwrap();

    let client = common::create_pg_client(&postgresql_container).await;

    let db_name = PgIdentifier::new("test_database").unwrap();

    // First creation
    let result = client.create_db(&db_name, false).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // Second creation
    let result = client.create_db(&db_name, false).await;

    assert! {
        matches!(result, Err(CreateDatabaseError::DbAlreadyExists {..})),
        "{result:?}"
    }
}
