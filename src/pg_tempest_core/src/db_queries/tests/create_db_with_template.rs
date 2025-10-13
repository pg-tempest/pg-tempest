use crate::db_queries::create_db::create_database;
use crate::db_queries::create_db_with_template::{
    CreateDbWithTemplateError, create_db_with_template,
};
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

    let template_name = PgIdentifier::new("test_template").unwrap();
    let db_name = PgIdentifier::new("test_database").unwrap();

    // Template creation
    let result = create_database(&pool, &template_name, false).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // First creation
    let result = create_db_with_template(&pool, &db_name, &template_name).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // Second creation
    let result = create_db_with_template(&pool, &db_name, &template_name).await;

    assert! {
        matches!(result, Err(CreateDbWithTemplateError::DbAlreadyExists {..})),
        "{result:?}"
    }
}

#[tokio::test]
async fn template_doesnt_exists() {
    let postgresql_container = testcontainers_modules::postgres::Postgres::default()
        .start()
        .await
        .unwrap();

    let pool = common::create_pg_pool(&postgresql_container).await;

    let template_name = PgIdentifier::new("test_template").unwrap();
    let db_name = PgIdentifier::new("test_database").unwrap();

    let result = create_db_with_template(&pool, &db_name, &template_name).await;

    assert! {
        matches!(result, Err(CreateDbWithTemplateError::TemplateDoesntExist {..})),
        "{result:?}"
    }
}
