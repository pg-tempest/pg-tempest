use crate::{db_queries::utils::db_doesnt_exist, models::value_types::pg_identifier::PgIdentifier};
use derive_more::Display;
use sqlx::PgPool;
use thiserror::Error;

pub async fn drop_db(pg_pool: &PgPool, db_name: &PgIdentifier) -> Result<(), DropDbError> {
    let query_result = sqlx::query(&format!(r#"DROP DATABASE "{db_name}""#))
        .execute(pg_pool)
        .await;

    match query_result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::Database(error)) if db_doesnt_exist(&error) => {
            Err(DropDbError::DbDoesntExists {
                db_name: db_name.clone(),
            })
        }
        Err(error) => Err(DropDbError::Unexpected {
            inner: error.into(),
        }),
    }
}

#[derive(Debug, Display, Error)]
#[display("{self:?}")]
pub enum DropDbError {
    DbDoesntExists { db_name: PgIdentifier },
    Unexpected { inner: anyhow::Error },
}
