use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct DbPoolConfigs {
    #[serde(default)]
    pub min_size: u8,
}
