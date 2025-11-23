use std::sync::Arc;

use crate::utils::{db_already_exists, db_doesnt_exist, wrong_object_type};
use async_trait::async_trait;
use pg_tempest_core::utils::adhoc_display::AdHocDisplay;
use pg_tempest_core::utils::errors::BoxDynError;
use pg_tempest_core::{
    configs::dbms_configs::DbmsConfigs,
    models::value_types::pg_identifier::PgIdentifier,
    pg_client::{AlterDbIsTemplateError, CreateDbError, Db, DropDbError, PgClient},
};
use sqlx::{
    FromRow, PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

pub struct PgClientImpl {
    pg_pool: PgPool,
}

impl PgClientImpl {
    pub fn new(configs: Arc<DbmsConfigs>) -> PgClientImpl {
        let pg_connect_options = PgConnectOptions::new_without_pgpass()
            .host(&configs.inner.host)
            .port(configs.inner.port)
            .database(&configs.database)
            .username(&configs.user)
            .password(&configs.password);

        let pg_pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(std::time::Duration::from_millis(500))
            .connect_lazy_with(pg_connect_options);

        PgClientImpl { pg_pool }
    }
}

#[async_trait]
impl PgClient for PgClientImpl {
    async fn alter_db_is_template(
        &self,
        db_name: PgIdentifier,
        is_template: bool,
    ) -> Result<(), AlterDbIsTemplateError> {
        let query_result = sqlx::query(&format!(
            r#"ALTER DATABASE "{db_name}" IS_TEMPLATE {is_template}"#
        ))
        .execute(&self.pg_pool)
        .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(error)) if db_doesnt_exist(&error) => {
                Err(AlterDbIsTemplateError::DbDoesNotExists {
                    db_name: db_name.clone(),
                })
            }
            Err(error) => Err(AlterDbIsTemplateError::Unexpected(error.into())),
        }
    }

    async fn create_db(
        &self,
        db_name: PgIdentifier,
        template_db_name: Option<PgIdentifier>,
        is_template: bool,
    ) -> Result<(), CreateDbError> {
        let query_result = sqlx::query(&format!(
            r#"
            CREATE DATABASE "{db_name}"
                TEMPLATE {}
                IS_TEMPLATE {is_template};
            "#,
            AdHocDisplay(|f| {
                match template_db_name {
                    None => f.write_str("DEFAULT"),
                    Some(ref db_name) => write!(f, r#""{db_name}""#),
                }
            })
        ))
        .execute(&self.pg_pool)
        .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(error)) if db_already_exists(&error) => {
                Err(CreateDbError::DbAlreadyExists {
                    db_name: db_name.clone(),
                })
            }
            Err(sqlx::Error::Database(error))
                if db_doesnt_exist(&error) && template_db_name.is_some() =>
            {
                Err(CreateDbError::TemplateDbDoesNotExist {
                    template_db_name: template_db_name.unwrap(),
                })
            }
            Err(error) => Err(CreateDbError::Unexpected(error.into())),
        }
    }

    async fn drop_db(&self, db_name: PgIdentifier) -> Result<(), DropDbError> {
        let query_result = sqlx::query(&format!(r#"DROP DATABASE "{db_name}""#))
            .execute(&self.pg_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(error)) if db_doesnt_exist(&error) => {
                Err(DropDbError::DbDoesNotExist {
                    db_name: db_name.clone(),
                })
            }
            Err(sqlx::Error::Database(error)) if wrong_object_type(&error) => {
                Err(DropDbError::DbIsTemplate {
                    db_name: db_name.clone(),
                })
            }
            Err(error) => Err(DropDbError::Unexpected(error.into())),
        }
    }

    async fn get_dbs(&self) -> Result<Vec<Db>, BoxDynError> {
        let rows: Vec<DbRow> = sqlx::query_as(
            r#"
            select
                oid,
                datname as "name",
                datistemplate as is_template,
                datdba as owner_oid,
                datallowconn as allow_connection
            from pg_database;
            "#,
        )
        .fetch_all(&self.pg_pool)
        .await?;

        let databases = rows
            .into_iter()
            .map(map_to_model)
            .collect::<Result<Vec<Db>, BoxDynError>>();

        databases
    }
}

#[derive(FromRow)]
struct DbRow {
    oid: sqlx::postgres::types::Oid,
    name: String,
    is_template: bool,
    owner_oid: sqlx::postgres::types::Oid,
    allow_connection: bool,
}

fn map_to_model(row: DbRow) -> Result<Db, BoxDynError> {
    Ok(Db {
        oid: row.oid.0,
        name: PgIdentifier::new(row.name)?,
        is_template: row.is_template,
        owner_oid: row.owner_oid.0,
        allow_connection: row.allow_connection,
    })
}
