use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct DbPoolConfigs {
    pub min_size: u8,
    pub creation_retries_delay_in_ms: u64,
}
