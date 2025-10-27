use std::sync::Arc;

use axum::{Router, routing::post};
use pg_tempest_core::PgTempestCore;

use crate::routes::templates::{
    extend_template_initialization::extend_template_initialization,
    fail_template_initialization::fail_template_initialization,
    finish_template_initialization::finish_template_initialization,
    start_template_initialization::start_template_initialization,
};

mod extend_template_initialization;
mod fail_template_initialization;
mod finish_template_initialization;
mod start_template_initialization;

pub fn create_templates_router(tempest_core: Arc<PgTempestCore>) -> Router {
    Router::new()
        .route(
            "/api/start-template-initialization",
            post(start_template_initialization),
        )
        .route(
            "/api/finish-template-initialization",
            post(finish_template_initialization),
        )
        .route(
            "/api/fail-template-initialization",
            post(fail_template_initialization),
        )
        .route(
            "/api/extend-template-initialization",
            post(extend_template_initialization),
        )
        .with_state(tempest_core)
}
