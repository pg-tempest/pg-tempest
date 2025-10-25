use serde::Deserialize;

#[derive(Deserialize)]
pub struct DbmsConfigs {
    pub inner: InnerDbmsConfigs,
    #[serde(default)]
    pub outer: OuterDbmsConfigs,
    pub database: Box<str>,
    pub user: Box<str>,
    pub password: Box<str>,
}

#[derive(Deserialize)]
pub struct InnerDbmsConfigs {
    pub host: Box<str>,
    pub port: u16,
}

#[derive(Deserialize, Default)]
pub struct OuterDbmsConfigs {
    pub host: Option<Box<str>>,
    pub port: Option<u16>,
}
