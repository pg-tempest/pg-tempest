use async_trait::async_trait;
use derive_more::Display;
use thiserror::Error;

use crate::models::value_types::pg_identifier::PgIdentifier;

#[async_trait]
pub trait PgClient: Send + Sync {
    async fn alter_db_is_template(
        &self,
        db_name: PgIdentifier,
        is_template: bool,
    ) -> Result<(), AlterDbIsTemplateError>;

    async fn create_db(
        &self,
        db_name: PgIdentifier,
        template_db_name: Option<PgIdentifier>,
        is_template: bool,
    ) -> Result<(), CreateDbError>;

    async fn drop_db(&self, db_name: PgIdentifier) -> Result<(), DropDbError>;

    async fn get_dbs(&self) -> anyhow::Result<Vec<Db>>;
}

#[derive(Debug, Display, Error)]
#[display("{self:?}")]
pub enum AlterDbIsTemplateError {
    DbDoesNotExists { db_name: PgIdentifier },
    Unexpected { inner: anyhow::Error },
}

#[derive(Debug, Display, Error)]
#[display("{self:?}")]
pub enum CreateDbError {
    DbAlreadyExists { db_name: PgIdentifier },
    TemplateDbDoesNotExist { template_db_name: PgIdentifier },
    Unexpected { inner: anyhow::Error },
}

#[derive(Debug, Display, Error)]
#[display("{self:?}")]
pub enum DropDbError {
    DbDoesNotExists { db_name: PgIdentifier },
    DbIsTemplate { db_name: PgIdentifier },
    Unexpected { inner: anyhow::Error },
}

pub struct Db {
    pub oid: u32,
    pub name: PgIdentifier,
    pub is_template: bool,
    pub owner_oid: u32,
    pub allow_connection: bool,
}
