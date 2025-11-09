use pg_tempest_core::{
    models::value_types::pg_identifier::PgIdentifier,
    pg_client::{CreateDbWithTemplateError, PgClient},
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

    let template_name = PgIdentifier::new("test_template").unwrap();
    let db_name = PgIdentifier::new("test_database").unwrap();

    // Template creation
    let result = client.create_db(&template_name, false).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // First creation
    let result = client
        .create_db_with_template(&db_name, &template_name)
        .await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // Second creation
    let result = client
        .create_db_with_template(&db_name, &template_name)
        .await;

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

    let client = common::create_pg_client(&postgresql_container).await;

    let template_name = PgIdentifier::new("test_template").unwrap();
    let db_name = PgIdentifier::new("test_database").unwrap();

    let result = client
        .create_db_with_template(&db_name, &template_name)
        .await;

    assert! {
        matches!(result, Err(CreateDbWithTemplateError::TemplateDoesntExist {..})),
        "{result:?}"
    }
}
