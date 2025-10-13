use crate::models::value_types::pg_identifier::PgIdentifier;

pub struct DbConnectionOptions {
    pub host: Box<str>,
    pub port: u16,
    pub username: Box<str>,
    pub password: Box<str>,
    pub database: PgIdentifier,
}
