use pg_tempest_core::{
    models::value_types::pg_identifier::PgIdentifier,
    pg_client::{DropDbError, PgClient},
};
use testcontainers::runners::AsyncRunner;

mod common;

#[tokio::test]
async fn db_double_drop() {
    let postgresql_container = testcontainers_modules::postgres::Postgres::default()
        .start()
        .await
        .unwrap();

    let client = common::create_pg_client(&postgresql_container).await;

    let db_name = PgIdentifier::new("test_database").unwrap();

    // DB creation
    let result = client.create_db(db_name.clone(), None, false).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // First drop
    let result = client.drop_db(db_name.clone()).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // Second drop
    let result = client.drop_db(db_name).await;

    assert! {
        matches!(result, Err(DropDbError::DbDoesNotExist {..})),
        "{result:?}"
    }
}

#[tokio::test]
async fn drop_template_db() {
    let postgresql_container = testcontainers_modules::postgres::Postgres::default()
        .start()
        .await
        .unwrap();

    let client = common::create_pg_client(&postgresql_container).await;

    let template_db_name = PgIdentifier::new("test_template").unwrap();

    // Template creation
    let result = client.create_db(template_db_name.clone(), None, true).await;

    assert! {
        matches!(result, Ok(_)),
        "{result:?}"
    }

    // Drop template
    let result = client.drop_db(template_db_name).await;

    assert! {
        matches!(result, Err(DropDbError::DbIsTemplate {..})),
        "{result:?}"
    }
}
