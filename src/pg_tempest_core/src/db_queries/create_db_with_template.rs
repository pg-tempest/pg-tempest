use crate::{
    db_queries::utils::{db_already_exists, db_doesnt_exist},
    models::value_types::pg_identifier::PgIdentifier,
};
use derive_more::Display;
use sqlx::PgPool;
use thiserror::Error;

pub async fn create_db_with_template(
    pg_pool: &PgPool,
    db_name: &PgIdentifier,
    template_name: &PgIdentifier,
) -> Result<(), CreateDbWithTemplateError> {
    let query_result = sqlx::query(&format!(
        r#"CREATE DATABASE "{db_name}" WITH TEMPLATE "{template_name}""#
    ))
    .execute(pg_pool)
    .await;

    match query_result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::Database(error)) if db_already_exists(&error) => {
            Err(CreateDbWithTemplateError::DbAlreadyExists {
                db_name: db_name.clone(),
            })
        }
        Err(sqlx::Error::Database(error)) if db_doesnt_exist(&error) => {
            Err(CreateDbWithTemplateError::TemplateDoesntExist {
                template_name: template_name.clone(),
            })
        }
        Err(error) => Err(CreateDbWithTemplateError::Unexpected {
            inner: error.into(),
        }),
    }
}

#[derive(Debug, Display, Error)]
#[display("{self:?}")]
pub enum CreateDbWithTemplateError {
    DbAlreadyExists { db_name: PgIdentifier },
    TemplateDoesntExist { template_name: PgIdentifier },
    Unexpected { inner: anyhow::Error },
}
