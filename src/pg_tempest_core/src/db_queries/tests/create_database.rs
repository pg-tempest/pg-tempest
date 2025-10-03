use testcontainers::runners::AsyncRunner;
use crate::db_queries::create_database::{create_database, CreateDatabaseError};
use crate::db_queries::tests::common;
use crate::models::pg_identifier::PgIdentifier;

#[tokio::test]
async fn db_double_creation() {
    let postgresql_container = testcontainers_modules::postgres::Postgres::default()
        .start()
        .await
        .unwrap();

    let pool = common::create_pg_pool(&postgresql_container).await;

    let db_name = PgIdentifier::new("test_database").unwrap();

    // First creation
    let result = create_database(&pool, &db_name).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // Second creation
    let result = create_database(&pool, &db_name).await;

    assert! {
        matches!(result, Err(CreateDatabaseError::DbAlreadyExists {..})),
        "{result:?}"
    }
}
