use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use pg_tempest_core::{
    features::templates::{
        TemplatesFeature,
        mark_template_initialization_as_failed::MarkTemplateInitializationAsFailedErrorResult,
    },
    models::value_types::template_hash::TemplateHash,
};
use serde::{Deserialize, Serialize};

use crate::dtos::json_response::JsonResponse;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkTemplateInitializationAsFailedRequestBody {
    template_hash: TemplateHash,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MarkTemplateInitializationAsFailedResponseBody {
    InitializationIsFailed {},
    TemplateWasNotFound {},
    InitializationIsFinished {},
}

pub async fn mark_template_initialization_as_failed(
    State(feature): State<Arc<TemplatesFeature>>,
    Json(request_body): Json<MarkTemplateInitializationAsFailedRequestBody>,
) -> JsonResponse<MarkTemplateInitializationAsFailedResponseBody> {
    let result = feature
        .mark_template_initialization_as_failed(request_body.template_hash)
        .await;

    match result {
        Ok(()) => JsonResponse {
            status_code: StatusCode::OK,
            body: MarkTemplateInitializationAsFailedResponseBody::InitializationIsFailed {},
        },
        Err(MarkTemplateInitializationAsFailedErrorResult::TemplateIsInitialized) => JsonResponse {
            status_code: StatusCode::CONFLICT,
            body: MarkTemplateInitializationAsFailedResponseBody::InitializationIsFinished {},
        },
        Err(MarkTemplateInitializationAsFailedErrorResult::TemplateWasNotFound) => JsonResponse {
            status_code: StatusCode::NOT_FOUND,
            body: MarkTemplateInitializationAsFailedResponseBody::TemplateWasNotFound {},
        },
    }
}
