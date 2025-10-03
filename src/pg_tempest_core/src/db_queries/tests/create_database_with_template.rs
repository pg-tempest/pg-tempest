use crate::db_queries::create_database::create_database;
use crate::db_queries::create_database_with_template::{
    CreateDatabaseWithTemplateError, create_database_with_template,
};
use crate::db_queries::tests::common;
use crate::models::pg_identifier::PgIdentifier;
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
    let result = create_database(&pool, &template_name).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // First creation
    let result = create_database_with_template(&pool, &db_name, &template_name).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // Second creation
    let result = create_database_with_template(&pool, &db_name, &template_name).await;

    assert! {
        matches!(result, Err(CreateDatabaseWithTemplateError::DbAlreadyExists {..})),
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

    let result = create_database_with_template(&pool, &db_name, &template_name).await;

    assert! {
        matches!(result, Err(CreateDatabaseWithTemplateError::TemplateDoesntExist {..})),
        "{result:?}"
    }
}
