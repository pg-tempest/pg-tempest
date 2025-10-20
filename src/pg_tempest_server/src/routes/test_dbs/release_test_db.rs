use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use pg_tempest_core::{
    features::test_dbs::{TestDbsFeature, release_test_db::ReleaseTestDbErrorResult},
    models::value_types::{template_hash::TemplateHash, test_db_id::TestDbId},
};
use serde::{Deserialize, Serialize};

use crate::dtos::json_response::JsonResponse;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseTestDbRequestBody {
    template_hash: TemplateHash,
    test_db_id: TestDbId,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ReleaseTestDbResponseBody {
    TestDbWasReleased {},
    TemplateWasNotFound {},
    TestDbWasNotFound {},
}

pub async fn release_test_db(
    State(feature): State<Arc<TestDbsFeature>>,
    Json(request_body): Json<ReleaseTestDbRequestBody>,
) -> JsonResponse<ReleaseTestDbResponseBody> {
    let result = feature
        .release_test_db(request_body.template_hash, request_body.test_db_id)
        .await;

    match result {
        Ok(_) => JsonResponse {
            status_code: StatusCode::OK,
            body: ReleaseTestDbResponseBody::TestDbWasReleased {},
        },
        Err(ReleaseTestDbErrorResult::TemplateWasNotFound) => JsonResponse {
            status_code: StatusCode::NOT_FOUND,
            body: ReleaseTestDbResponseBody::TemplateWasNotFound {},
        },
        Err(ReleaseTestDbErrorResult::TestDbWasNotFound) => JsonResponse {
            status_code: StatusCode::NOT_FOUND,
            body: ReleaseTestDbResponseBody::TestDbWasNotFound {},
        },
    }
}
