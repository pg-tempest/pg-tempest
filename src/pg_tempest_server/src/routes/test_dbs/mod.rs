mod extend_test_db_usage;
mod finish_test_db_usage;
mod get_test_db;

use std::sync::Arc;

use axum::{Router, routing::post};
use pg_tempest_core::PgTempestCore;

use crate::routes::test_dbs::{
    extend_test_db_usage::extend_test_db_usage, finish_test_db_usage::finish_test_db_usage,
    get_test_db::get_test_db,
};

pub fn create_test_dbs_router(tempest_core: Arc<PgTempestCore>) -> Router {
    Router::new()
        .route("/api/get-test-db", post(get_test_db))
        .route("/api/extend-test-db-usage", post(extend_test_db_usage))
        .route("/api/finish-test-db-usage", post(finish_test_db_usage))
        .with_state(tempest_core)
}
