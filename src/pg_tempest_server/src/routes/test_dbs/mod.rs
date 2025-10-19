mod get_test_db;

use std::sync::Arc;

use axum::{Router, routing::post};
use pg_tempest_core::features::test_dbs::TestDbsFeature;

use crate::routes::test_dbs::get_test_db::get_test_db;

pub fn create_test_dbs_router(test_dbs_feature: Arc<TestDbsFeature>) -> Router {
    Router::new()
        .route("/test-dbs/get", post(get_test_db))
        .with_state(test_dbs_feature)
}
