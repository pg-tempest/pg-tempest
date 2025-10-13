use crate::{
    db_queries::utils::db_already_exists, models::value_types::pg_identifier::PgIdentifier,
};
use derive_more::Display;
use sqlx::PgPool;
use thiserror::Error;

pub async fn create_database(
    pg_pool: &PgPool,
    db_name: &PgIdentifier,
    is_template: bool,
) -> Result<(), CreateDatabaseError> {
    let query_result = sqlx::query(&format!(
        r#"
        CREATE DATABASE "{db_name}"
            WITH
            IS_TEMPLATE = {is_template};
        "#
    ))
    .execute(pg_pool)
    .await;

    match query_result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::Database(error)) if db_already_exists(&error) => {
            Err(CreateDatabaseError::DbAlreadyExists {
                db_name: db_name.clone(),
            })
        }
        Err(error) => Err(CreateDatabaseError::Unexpected {
            inner: error.into(),
        }),
    }
}

#[derive(Debug, Display, Error)]
#[display("{self:?}")]
pub enum CreateDatabaseError {
    DbAlreadyExists { db_name: PgIdentifier },
    Unexpected { inner: anyhow::Error },
}
