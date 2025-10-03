use crate::models::template_database::{TemplateDatabase, TemplateHash};
use static_assertions::assert_impl_all;
use std::collections::HashMap;
use std::error::Error;
use std::ops::DerefMut;
use tokio::sync::Mutex;

pub struct TemplateRepository {
    templates: Mutex<HashMap<TemplateHash, TemplateDatabase>>,
}

impl TemplateRepository {
    pub async fn get_or_insert(
        &self,
        template_database: TemplateDatabase,
    ) -> Result<TemplateDatabase, Box<dyn Error>> {
        let mut guard = self.templates.lock().await;
        let templates = guard.deref_mut();

        let hash = &template_database.hash;

        if let Some(existed_template) = templates.get(hash) {
            return Ok(existed_template.clone());
        }

        templates.insert(hash.clone(), template_database.clone());

        Ok(template_database)
    }
}

assert_impl_all!(TemplateRepository: Send, Sync);
