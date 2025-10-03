use crate::db_queries::utils::db_doesnt_exist;
use crate::models::pg_identifier::PgIdentifier;
use derive_more::Display;
use sqlx::PgPool;
use std::error::Error;

pub async fn drop_database(
    pg_pool: &PgPool,
    db_name: &PgIdentifier,
) -> Result<(), DropDatabaseError> {
    let query_result = sqlx::query(&format!("DROP DATABASE {db_name}"))
        .bind(db_name.as_ref())
        .execute(pg_pool)
        .await;

    match query_result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::Database(error)) if db_doesnt_exist(&error) => {
            Err(DropDatabaseError::DbDoesntExists {
                db_name: db_name.clone(),
            })
        }
        Err(error) => Err(DropDatabaseError::Unexpected {
            inner: error.into(),
        }),
    }
}

#[derive(Debug, Display)]
#[display("{self:?}")]
pub enum DropDatabaseError {
    DbDoesntExists { db_name: PgIdentifier },
    Unexpected { inner: Box<dyn Error> },
}
