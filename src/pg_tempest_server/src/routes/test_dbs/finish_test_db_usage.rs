use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use pg_tempest_core::{
    PgTempestCore,
    features::test_dbs::finish_test_db_usage::FinishTestDbUsageErrorResult,
    models::value_types::{template_hash::TemplateHash, test_db_id::TestDbId},
};
use serde::{Deserialize, Serialize};

use crate::dtos::json_response::JsonResponse;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishTestDbUsageRequestBody {
    template_hash: TemplateHash,
    test_db_id: TestDbId,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum FinishTestDbUsageResponseBody {
    TestDbWasReleased {},
    TemplateWasNotFound {},
    TestDbWasNotFound {},
    TestDbIsNotUsed {},
}

pub async fn finish_test_db_usage(
    State(tempest_core): State<Arc<PgTempestCore>>,
    Json(request_body): Json<FinishTestDbUsageRequestBody>,
) -> JsonResponse<FinishTestDbUsageResponseBody> {
    let result = tempest_core
        .finish_test_db_usage(request_body.template_hash, request_body.test_db_id)
        .await;

    match result {
        Ok(_) => JsonResponse {
            status_code: StatusCode::OK,
            body: FinishTestDbUsageResponseBody::TestDbWasReleased {},
        },
        Err(FinishTestDbUsageErrorResult::TemplateWasNotFound) => JsonResponse {
            status_code: StatusCode::NOT_FOUND,
            body: FinishTestDbUsageResponseBody::TemplateWasNotFound {},
        },
        Err(FinishTestDbUsageErrorResult::TestDbWasNotFound) => JsonResponse {
            status_code: StatusCode::NOT_FOUND,
            body: FinishTestDbUsageResponseBody::TestDbWasNotFound {},
        },
        Err(FinishTestDbUsageErrorResult::TestDbIsNotUsed) => JsonResponse {
            status_code: StatusCode::CONFLICT,
            body: FinishTestDbUsageResponseBody::TestDbIsNotUsed {},
        },
    }
}
