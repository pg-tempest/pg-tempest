use derive_more::Display;
use sqlx::PgPool;
use thiserror::Error;

use crate::{db_queries::utils::db_doesnt_exist, models::value_types::pg_identifier::PgIdentifier};

pub async fn alter_db_is_template(
    pg_pool: &PgPool,
    db_name: &PgIdentifier,
    is_template: bool,
) -> Result<(), AlterDbIsTemplateError> {
    let query_result = sqlx::query(&format!(
        r#"ALTER DATABASE "{db_name}" IS_TEMPLATE {is_template}"#
    ))
    .execute(pg_pool)
    .await;

    match query_result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::Database(error)) if db_doesnt_exist(&error) => {
            Err(AlterDbIsTemplateError::DbDoesntExists {
                db_name: db_name.clone(),
            })
        }
        Err(error) => Err(AlterDbIsTemplateError::Unexpected {
            inner: error.into(),
        }),
    }
}

#[derive(Debug, Display, Error)]
#[display("{self:?}")]
pub enum AlterDbIsTemplateError {
    DbDoesntExists { db_name: PgIdentifier },
    Unexpected { inner: anyhow::Error },
}
