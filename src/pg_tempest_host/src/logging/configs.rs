use derive_more::FromStr;
use serde::Deserialize;
use tracing::Level;

#[derive(Deserialize, Default)]
#[serde(from = "LoggingConfigsDto")]
pub struct LoggingConfigs {
    pub core: Option<Level>,
    pub db_queries: Option<Level>,
    pub server: Option<Level>,
}

#[derive(Deserialize)]
struct LoggingConfigsDto {
    core: Option<LevelDto>,
    db_queries: Option<LevelDto>,
    server: Option<LevelDto>,
}

impl From<LoggingConfigsDto> for LoggingConfigs {
    fn from(value: LoggingConfigsDto) -> Self {
        LoggingConfigs {
            core: value.core.map(|x| x.0),
            db_queries: value.db_queries.map(|x| x.0),
            server: value.server.map(|x| x.0),
        }
    }
}

#[derive(FromStr, Deserialize)]
#[serde(try_from = "Box<str>")]
struct LevelDto(Level);

impl TryFrom<Box<str>> for LevelDto {
    type Error = <LevelDto as std::str::FromStr>::Err;

    fn try_from(value: Box<str>) -> Result<Self, Self::Error> {
        value.parse()
    }
}
