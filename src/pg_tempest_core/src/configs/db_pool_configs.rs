use serde::Deserialize;

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DbPoolConfigs {
    #[serde(default)]
    pub min_size: u8,
}
