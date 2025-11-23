use crate::configs::template_initialization_configs::TemplateInitializationConfigs;
use crate::models::value_types::pg_identifier::PgIdentifier;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct TemplatesConfigs {
    pub initialization: Arc<TemplateInitializationConfigs>,
    pub parent_template_db_name: Option<PgIdentifier>,
}
