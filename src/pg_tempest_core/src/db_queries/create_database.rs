use derive_more::Display;
use sqlx::PgPool;
use std::error::Error;
use crate::db_queries::utils::db_already_exists;
use crate::models::pg_identifier::PgIdentifier;

pub async fn create_database(
    pg_pool: &PgPool,
    db_name: &PgIdentifier,
) -> Result<(), CreateDatabaseError> {
    let query_result = sqlx::query(&format!("CREATE DATABASE {db_name}"))
        .bind(db_name.as_ref())
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

#[derive(Debug, Display)]
#[display("{self:?}")]
pub enum CreateDatabaseError {
    DbAlreadyExists { db_name: PgIdentifier },
    Unexpected { inner: Box<dyn Error> },
}
