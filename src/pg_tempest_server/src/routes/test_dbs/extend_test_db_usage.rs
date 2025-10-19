use std::{sync::Arc, time::Duration};

use axum::{Json, extract::State};
use chrono::{DateTime, Utc};
use pg_tempest_core::{
    features::test_dbs::{TestDbsFeature, extend_test_db_usage::ExtendTestDbUsageErrorResult},
    models::value_types::{template_hash::TemplateHash, test_db_id::TestDbId},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendTestDbUsageRequestBody {
    template_hash: TemplateHash,
    test_db_id: TestDbId,
    additional_time_in_seconds: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendTestDbUsageOkResponse {
    new_usage_deadline: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ExtendTestDbUsageErrorResponse {
    TemplateWasNotFound {},
    TestDbWasNotFound {},
    TestDbIsNotInUse {},
    TestDbIsCorrupted {},
}

pub async fn extend_test_db_usage(
    State(feature): State<Arc<TestDbsFeature>>,
    Json(request_body): Json<ExtendTestDbUsageRequestBody>,
) -> Result<Json<ExtendTestDbUsageOkResponse>, Json<ExtendTestDbUsageErrorResponse>> {
    let result = feature
        .extend_test_db_usage(
            request_body.template_hash,
            request_body.test_db_id,
            Duration::from_secs(request_body.additional_time_in_seconds),
        )
        .await;

    match result {
        Ok(result) => Ok(Json(ExtendTestDbUsageOkResponse {
            new_usage_deadline: result.new_usage_deadline,
        })),
        Err(ExtendTestDbUsageErrorResult::TemplateWasNotFound) => {
            Err(Json(ExtendTestDbUsageErrorResponse::TemplateWasNotFound {}))
        }
        Err(ExtendTestDbUsageErrorResult::TestDbWasNotFound) => {
            Err(Json(ExtendTestDbUsageErrorResponse::TestDbWasNotFound {}))
        }
        Err(ExtendTestDbUsageErrorResult::TestDbIsNotInUse) => {
            Err(Json(ExtendTestDbUsageErrorResponse::TestDbIsNotInUse {}))
        }
        Err(ExtendTestDbUsageErrorResult::TestDbIsCorrupted) => {
            Err(Json(ExtendTestDbUsageErrorResponse::TestDbIsCorrupted {}))
        }
    }
}
