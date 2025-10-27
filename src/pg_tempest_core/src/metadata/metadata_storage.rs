use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    metadata::template_metadata::TemplateMetadata, models::value_types::template_hash::TemplateHash,
};

pub struct MetadataStorage {
    template_metadatas_by_template_hash:
        Mutex<HashMap<TemplateHash, Arc<Mutex<Option<TemplateMetadata>>>>>,
}

impl MetadataStorage {
    pub fn new() -> MetadataStorage {
        MetadataStorage {
            template_metadatas_by_template_hash: Mutex::new(HashMap::new()),
        }
    }

    pub async fn execute_under_lock<TResult>(
        &self,
        template_hash: TemplateHash,
        action: impl FnOnce(&mut Option<TemplateMetadata>) -> TResult,
    ) -> TResult {
        let template_metadata = {
            let mut hash_map = self.template_metadatas_by_template_hash.lock().await;

            hash_map
                .entry(template_hash)
                .or_insert_with(|| Arc::new(Mutex::new(None)))
                .clone()
        };

        let mut template_metadata = template_metadata.lock().await;

        action(&mut *template_metadata)
    }

    pub async fn get_all_template_hashes(&self) -> Vec<TemplateHash> {
        let hash_map = self.template_metadatas_by_template_hash.lock().await;

        hash_map.iter().map(|(hash, _)| *hash).collect()
    }
}
