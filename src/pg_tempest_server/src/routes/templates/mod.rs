use std::sync::Arc;

use axum::{Router, routing::post};
use pg_tempest_core::features::templates::TemplatesFeature;

use crate::routes::templates::start_template_initialization::start_template_initialization;

mod start_template_initialization;

pub fn create_templates_router(templates_feature: Arc<TemplatesFeature>) -> Router {
    Router::new()
        .route(
            "/templates/start-initialization",
            post(start_template_initialization),
        )
        .with_state(templates_feature)
}
