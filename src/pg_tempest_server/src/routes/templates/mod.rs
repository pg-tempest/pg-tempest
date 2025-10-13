use std::sync::Arc;

use axum::{Router, routing::post};
use pg_tempest_core::features::templates::TemplatesFeature;

use crate::routes::templates::{
    finish_template_initialization::finish_template_initialization,
    mark_template_initialization_as_failed::mark_template_initialization_as_failed,
    start_template_initialization::start_template_initialization,
};

mod finish_template_initialization;
mod mark_template_initialization_as_failed;
mod start_template_initialization;

pub fn create_templates_router(templates_feature: Arc<TemplatesFeature>) -> Router {
    Router::new()
        .route(
            "/templates/start-initialization",
            post(start_template_initialization),
        )
        .route(
            "/templates/finish-initialization",
            post(finish_template_initialization),
        )
        .route(
            "/templates/mark-initialization-as-failed",
            post(mark_template_initialization_as_failed),
        )
        .with_state(templates_feature)
}
