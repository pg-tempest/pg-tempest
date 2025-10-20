use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use pg_tempest_core::{
    features::templates::{
        TemplatesFeature, finish_template_initialization::FinishTemplateInitializationErrorResult,
    },
    models::value_types::template_hash::TemplateHash,
};
use serde::{Deserialize, Serialize};

use crate::dtos::json_response::JsonResponse;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishTemplateInitializationRequestBody {
    template_hash: TemplateHash,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FinishTemplateInitializationResponseBody {
    InitializationIsFinished {},
    TemplateWasNotFound {},
    InitializationIsFailed {},
}

pub async fn finish_template_initialization(
    State(feature): State<Arc<TemplatesFeature>>,
    Json(request_body): Json<FinishTemplateInitializationRequestBody>,
) -> JsonResponse<FinishTemplateInitializationResponseBody> {
    let result = feature
        .finish_template_initialization(request_body.template_hash)
        .await;

    match result {
        Ok(()) => JsonResponse {
            status_code: StatusCode::OK,
            body: FinishTemplateInitializationResponseBody::InitializationIsFinished {},
        },
        Err(FinishTemplateInitializationErrorResult::InitializationIsFailed) => JsonResponse {
            status_code: StatusCode::CONFLICT,
            body: FinishTemplateInitializationResponseBody::InitializationIsFailed {},
        },
        Err(FinishTemplateInitializationErrorResult::TemplateWasNotFound) => JsonResponse {
            status_code: StatusCode::NOT_FOUND,
            body: FinishTemplateInitializationResponseBody::TemplateWasNotFound {},
        },
    }
}
