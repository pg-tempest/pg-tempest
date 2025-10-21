use std::{sync::Arc, time::Duration};

use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use pg_tempest_core::{
    features::test_dbs::{TestDbsFeature, extend_test_db_usage::ExtendTestDbUsageErrorResult},
    models::value_types::{template_hash::TemplateHash, test_db_id::TestDbId},
};
use serde::{Deserialize, Serialize};

use crate::dtos::json_response::JsonResponse;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendTestDbUsageRequestBody {
    template_hash: TemplateHash,
    test_db_id: TestDbId,
    additional_time_in_seconds: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ExtendTestDbUsageResponseBody {
    UsageWasExtended { new_usage_deadline: DateTime<Utc> },
    TemplateWasNotFound {},
    TestDbWasNotFound {},
    TestDbIsNotInUse {},
    TestDbIsCorrupted {},
}

pub async fn extend_test_db_usage(
    State(feature): State<Arc<TestDbsFeature>>,
    Json(request_body): Json<ExtendTestDbUsageRequestBody>,
) -> JsonResponse<ExtendTestDbUsageResponseBody> {
    let result = feature
        .extend_test_db_usage(
            request_body.template_hash,
            request_body.test_db_id,
            Duration::from_secs(request_body.additional_time_in_seconds),
        )
        .await;

    match result {
        Ok(result) => JsonResponse {
            status_code: StatusCode::OK,
            body: ExtendTestDbUsageResponseBody::UsageWasExtended {
                new_usage_deadline: result.new_usage_deadline,
            },
        },
        Err(ExtendTestDbUsageErrorResult::TemplateWasNotFound) => JsonResponse {
            status_code: StatusCode::NOT_FOUND,
            body: ExtendTestDbUsageResponseBody::TemplateWasNotFound {},
        },
        Err(ExtendTestDbUsageErrorResult::TestDbWasNotFound) => JsonResponse {
            status_code: StatusCode::NOT_FOUND,
            body: ExtendTestDbUsageResponseBody::TestDbWasNotFound {},
        },
        Err(ExtendTestDbUsageErrorResult::TestDbIsNotInUse) => JsonResponse {
            status_code: StatusCode::BAD_REQUEST,
            body: ExtendTestDbUsageResponseBody::TestDbIsNotInUse {},
        },
        Err(ExtendTestDbUsageErrorResult::TestDbIsCorrupted) => JsonResponse {
            status_code: StatusCode::BAD_REQUEST,
            body: ExtendTestDbUsageResponseBody::TestDbIsCorrupted {},
        },
    }
}
