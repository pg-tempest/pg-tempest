mod extend_test_db_usage;
mod get_test_db;
mod release_test_db;

use std::sync::Arc;

use axum::{Router, routing::post};
use pg_tempest_core::PgTempestCore;

use crate::routes::test_dbs::{
    extend_test_db_usage::extend_test_db_usage, get_test_db::get_test_db,
    release_test_db::release_test_db,
};

pub fn create_test_dbs_router(tempest_core: Arc<PgTempestCore>) -> Router {
    Router::new()
        .route("/test-dbs/get", post(get_test_db))
        .route("/test-dbs/extend-usage", post(extend_test_db_usage))
        .route("/test-dbs/release", post(release_test_db))
        .with_state(tempest_core)
}
