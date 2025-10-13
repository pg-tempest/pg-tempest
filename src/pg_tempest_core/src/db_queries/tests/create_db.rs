use crate::db_queries::create_db::{CreateDatabaseError, create_db};
use crate::db_queries::tests::common;
use crate::models::value_types::pg_identifier::PgIdentifier;
use testcontainers::runners::AsyncRunner;

#[tokio::test]
async fn db_double_creation() {
    let postgresql_container = testcontainers_modules::postgres::Postgres::default()
        .start()
        .await
        .unwrap();

    let pool = common::create_pg_pool(&postgresql_container).await;

    let db_name = PgIdentifier::new("test_database").unwrap();

    // First creation
    let result = create_db(&pool, &db_name, false).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // Second creation
    let result = create_db(&pool, &db_name, false).await;

    assert! {
        matches!(result, Err(CreateDatabaseError::DbAlreadyExists {..})),
        "{result:?}"
    }
}
