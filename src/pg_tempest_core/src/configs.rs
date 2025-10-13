use std::sync::Arc;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreConfigs {
    pub dbms: Arc<DbmsConfigs>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbmsConfigs {
    pub host: Box<str>,
    pub port: u16,
    pub database: Box<str>,
    pub user: Box<str>,
    pub password: Box<str>,
}
