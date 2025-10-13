use crate::db_queries::create_db::create_database;
use crate::db_queries::create_db_with_template::create_db_with_template;
use crate::db_queries::drop_db::{DropDbError, drop_db};
use crate::db_queries::tests::common;
use crate::models::value_types::pg_identifier::PgIdentifier;
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
    let result = create_database(&pool, &db_name, false).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // First drop
    let result = drop_db(&pool, &db_name).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // Second drop
    let result = drop_db(&pool, &db_name).await;

    assert! {
        matches!(result, Err(DropDbError::DbDoesntExists {..})),
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
    let result = create_database(&pool, &template_name, false).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // Db creation
    let result = create_db_with_template(&pool, &db_name, &template_name).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // Drop template
    let result = drop_db(&pool, &template_name).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }
}
