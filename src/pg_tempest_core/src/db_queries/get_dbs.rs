use sqlx::{PgPool, prelude::FromRow};

use crate::models::value_types::pg_identifier::PgIdentifier;

pub async fn get_dbs(pg_pool: &PgPool) -> anyhow::Result<Vec<Db>> {
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
    .fetch_all(pg_pool)
    .await?;

    let databases = rows
        .into_iter()
        .map(map_to_model)
        .collect::<anyhow::Result<Vec<Db>>>();

    databases
}

pub struct Db {
    pub oid: u32,
    pub name: PgIdentifier,
    pub is_template: bool,
    pub owner_oid: u32,
    pub allow_connection: bool,
}

#[derive(FromRow)]
struct DbRow {
    oid: sqlx::postgres::types::Oid,
    name: String,
    is_template: bool,
    owner_oid: sqlx::postgres::types::Oid,
    allow_connection: bool,
}

fn map_to_model(row: DbRow) -> anyhow::Result<Db> {
    Ok(Db {
        oid: row.oid.0,
        name: PgIdentifier::new(row.name)?,
        is_template: row.is_template,
        owner_oid: row.owner_oid.0,
        allow_connection: row.allow_connection,
    })
}
