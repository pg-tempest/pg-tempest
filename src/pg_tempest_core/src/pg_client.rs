use async_trait::async_trait;
use derive_more::Display;
use thiserror::Error;

use crate::models::value_types::pg_identifier::PgIdentifier;

#[async_trait]
pub trait PgClient: Send + Sync {
    async fn alter_db_is_template(
        &self,
        db_name: &PgIdentifier,
        is_template: bool,
    ) -> Result<(), AlterDbIsTemplateError>;

    async fn create_db_with_template(
        &self,
        db_name: &PgIdentifier,
        template_name: &PgIdentifier,
    ) -> Result<(), CreateDbWithTemplateError>;

    async fn create_db(
        &self,
        db_name: &PgIdentifier,
        is_template: bool,
    ) -> Result<(), CreateDatabaseError>;

    async fn drop_db(&self, db_name: &PgIdentifier) -> Result<(), DropDbError>;

    async fn get_dbs(&self) -> anyhow::Result<Vec<Db>>;
}

#[derive(Debug, Display, Error)]
#[display("{self:?}")]
pub enum AlterDbIsTemplateError {
    DbDoesntExists { db_name: PgIdentifier },
    Unexpected { inner: anyhow::Error },
}

#[derive(Debug, Display, Error)]
#[display("{self:?}")]
pub enum CreateDbWithTemplateError {
    DbAlreadyExists { db_name: PgIdentifier },
    TemplateDoesntExist { template_name: PgIdentifier },
    Unexpected { inner: anyhow::Error },
}

#[derive(Debug, Display, Error)]
#[display("{self:?}")]
pub enum CreateDatabaseError {
    DbAlreadyExists { db_name: PgIdentifier },
    Unexpected { inner: anyhow::Error },
}

#[derive(Debug, Display, Error)]
#[display("{self:?}")]
pub enum DropDbError {
    DbDoesntExists { db_name: PgIdentifier },
    Unexpected { inner: anyhow::Error },
}

pub struct Db {
    pub oid: u32,
    pub name: PgIdentifier,
    pub is_template: bool,
    pub owner_oid: u32,
    pub allow_connection: bool,
}
