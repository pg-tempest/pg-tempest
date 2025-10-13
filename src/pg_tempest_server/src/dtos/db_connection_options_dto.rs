use pg_tempest_core::models::{
    db_connection_options::DbConnectionOptions, value_types::pg_identifier::PgIdentifier,
};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DbConnectionOptionsDto {
    pub host: Box<str>,
    pub port: u16,
    pub username: Box<str>,
    pub password: Box<str>,
    pub database: PgIdentifier,
}

impl From<DbConnectionOptions> for DbConnectionOptionsDto {
    fn from(value: DbConnectionOptions) -> Self {
        DbConnectionOptionsDto {
            host: value.host,
            port: value.port,
            username: value.username,
            password: value.password,
            database: value.database,
        }
    }
}
