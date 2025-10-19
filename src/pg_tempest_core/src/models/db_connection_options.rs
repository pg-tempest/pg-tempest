use crate::{DbmsConfigs, models::value_types::pg_identifier::PgIdentifier};

pub struct DbConnectionOptions {
    pub host: Box<str>,
    pub port: u16,
    pub username: Box<str>,
    pub password: Box<str>,
    pub database: PgIdentifier,
}

impl DbConnectionOptions {
    pub fn new_outer(configs: &DbmsConfigs, database: PgIdentifier) -> DbConnectionOptions {
        DbConnectionOptions {
            host: configs
                .outer
                .host
                .clone()
                .unwrap_or_else(|| configs.inner.host.clone()),
            port: configs.outer.port.unwrap_or_else(|| configs.inner.port),
            username: configs.user.clone(),
            password: configs.password.clone(),
            database: database.into(),
        }
    }
}
