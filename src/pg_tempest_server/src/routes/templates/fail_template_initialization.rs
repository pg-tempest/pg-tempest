use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use pg_tempest_core::{
    PgTempestCore,
    features::templates::fail_template_initialization::FailTemplateInitializationError,
    models::value_types::template_hash::TemplateHash,
};
use serde::{Deserialize, Serialize};

use crate::dtos::json_response::JsonResponse;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FailTemplateInitializationRequestBody {
    template_hash: TemplateHash,
    reason: Option<Arc<str>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum FailTemplateInitializationResponseBody {
    InitializationIsFailed {},
    TemplateWasNotFound {},
    InitializationIsNotStarted {},
    InitializationIsFinished {},
}

pub async fn fail_template_initialization(
    State(tempest_core): State<Arc<PgTempestCore>>,
    Json(request_body): Json<FailTemplateInitializationRequestBody>,
) -> JsonResponse<FailTemplateInitializationResponseBody> {
    let result = tempest_core
        .fail_template_initialization(request_body.template_hash, request_body.reason)
        .await;

    match result {
        Ok(()) => JsonResponse {
            status_code: StatusCode::OK,
            body: FailTemplateInitializationResponseBody::InitializationIsFailed {},
        },
        Err(FailTemplateInitializationError::InitializationIsFinished) => JsonResponse {
            status_code: StatusCode::CONFLICT,
            body: FailTemplateInitializationResponseBody::InitializationIsFinished {},
        },
        Err(FailTemplateInitializationError::TemplateWasNotFound { .. }) => JsonResponse {
            status_code: StatusCode::NOT_FOUND,
            body: FailTemplateInitializationResponseBody::TemplateWasNotFound {},
        },
        Err(FailTemplateInitializationError::InitializationIsNotStarted) => JsonResponse {
            status_code: StatusCode::CONFLICT,
            body: FailTemplateInitializationResponseBody::InitializationIsNotStarted {},
        },
    }
}
