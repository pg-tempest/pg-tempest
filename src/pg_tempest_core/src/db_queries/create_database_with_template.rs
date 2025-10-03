use crate::db_queries::utils::{db_already_exists, db_doesnt_exist};
use crate::models::pg_identifier::PgIdentifier;
use derive_more::Display;
use sqlx::PgPool;
use std::error::Error;

pub async fn create_database_with_template(
    pg_pool: &PgPool,
    db_name: &PgIdentifier,
    template_name: &PgIdentifier,
) -> Result<(), CreateDatabaseWithTemplateError> {
    let query_result = sqlx::query(&format!(
        "CREATE DATABASE {db_name} WITH TEMPLATE {template_name}"
    ))
    .bind(db_name.as_ref())
    .execute(pg_pool)
    .await;

    match query_result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::Database(error)) if db_already_exists(&error) => {
            Err(CreateDatabaseWithTemplateError::DbAlreadyExists {
                db_name: db_name.clone(),
            })
        }
        Err(sqlx::Error::Database(error)) if db_doesnt_exist(&error) => {
            Err(CreateDatabaseWithTemplateError::TemplateDoesntExist {
                template_name: template_name.clone(),
            })
        }
        Err(error) => Err(CreateDatabaseWithTemplateError::Unexpected {
            inner: error.into(),
        }),
    }
}

#[derive(Debug, Display)]
#[display("{self:?}")]
pub enum CreateDatabaseWithTemplateError {
    DbAlreadyExists { db_name: PgIdentifier },
    TemplateDoesntExist { template_name: PgIdentifier },
    Unexpected { inner: Box<dyn Error> },
}
