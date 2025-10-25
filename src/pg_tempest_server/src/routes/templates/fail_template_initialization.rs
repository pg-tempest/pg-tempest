use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use pg_tempest_core::{
    PgTempestCore,
    features::templates::fail_template_initialization::FailTemplateInitializationErrorResult,
    models::value_types::template_hash::TemplateHash,
};
use serde::{Deserialize, Serialize};

use crate::dtos::json_response::JsonResponse;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FailTemplateInitializationRequestBody {
    template_hash: TemplateHash,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum FailTemplateInitializationResponseBody {
    InitializationIsFailed {},
    TemplateWasNotFound {},
    InitializationIsFinished {},
}

pub async fn fail_template_initialization(
    State(tempest_core): State<Arc<PgTempestCore>>,
    Json(request_body): Json<FailTemplateInitializationRequestBody>,
) -> JsonResponse<FailTemplateInitializationResponseBody> {
    let result = tempest_core
        .fail_template_initialization(request_body.template_hash)
        .await;

    match result {
        Ok(()) => JsonResponse {
            status_code: StatusCode::OK,
            body: FailTemplateInitializationResponseBody::InitializationIsFailed {},
        },
        Err(FailTemplateInitializationErrorResult::TemplateIsInitialized) => JsonResponse {
            status_code: StatusCode::CONFLICT,
            body: FailTemplateInitializationResponseBody::InitializationIsFinished {},
        },
        Err(FailTemplateInitializationErrorResult::TemplateWasNotFound) => JsonResponse {
            status_code: StatusCode::NOT_FOUND,
            body: FailTemplateInitializationResponseBody::TemplateWasNotFound {},
        },
    }
}
