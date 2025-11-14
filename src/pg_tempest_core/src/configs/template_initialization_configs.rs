use serde::Deserialize;

#[derive(Deserialize)]
pub struct TemplateInitializationConfigs {
    pub long_polling_timeout_ms: u64,
    pub max_deadline_handling_delay_ms: u64,
}
