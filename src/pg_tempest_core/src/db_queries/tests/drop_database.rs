use crate::db_queries::create_database::create_database;
use crate::db_queries::create_database_with_template::create_database_with_template;
use crate::db_queries::drop_database::{DropDatabaseError, drop_database};
use crate::db_queries::tests::common;
use crate::models::pg_identifier::PgIdentifier;
use testcontainers::runners::AsyncRunner;

#[tokio::test]
async fn db_double_drop() {
    let postgresql_container = testcontainers_modules::postgres::Postgres::default()
        .start()
        .await
        .unwrap();

    let pool = common::create_pg_pool(&postgresql_container).await;

    let db_name = PgIdentifier::new("test_database").unwrap();

    // DB creation
    let result = create_database(&pool, &db_name).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // First drop
    let result = drop_database(&pool, &db_name).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // Second drop
    let result = drop_database(&pool, &db_name).await;

    assert! {
        matches!(result, Err(DropDatabaseError::DbDoesntExists {..})),
        "{result:?}"
    }
}

#[tokio::test]
async fn drop_used_template() {
    let postgresql_container = testcontainers_modules::postgres::Postgres::default()
        .start()
        .await
        .unwrap();

    let pool = common::create_pg_pool(&postgresql_container).await;

    let template_name = PgIdentifier::new("test_template").unwrap();
    let db_name = PgIdentifier::new("test_database").unwrap();

    // Template creation
    let result = create_database(&pool, &template_name).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // Db creation
    let result = create_database_with_template(&pool, &db_name, &template_name).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // Drop template
    let result = drop_database(&pool, &template_name).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }
}
