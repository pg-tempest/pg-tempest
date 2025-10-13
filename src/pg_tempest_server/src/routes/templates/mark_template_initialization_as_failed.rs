use std::sync::Arc;

use axum::{Json, extract::State};
use pg_tempest_core::{
    features::templates::{
        TemplatesFeature,
        mark_template_initialization_as_failed::MarkTemplateInitializationAsFailedErrorResult,
    },
    models::value_types::template_hash::TemplateHash,
};
use serde::{Deserialize, Serialize};

use crate::dtos::empty_dto::EmptyDto;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkTemplateInitializationAsFailedRequestBody {
    template_hash: TemplateHash,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MarkTemplateInitializationAsFailedErrorResponseBody {
    TemplateWasNotFound {},
    TemplateIsInitialized {},
}

pub async fn mark_template_initialization_as_failed(
    State(feature): State<Arc<TemplatesFeature>>,
    Json(request_body): Json<MarkTemplateInitializationAsFailedRequestBody>,
) -> Result<Json<EmptyDto>, Json<MarkTemplateInitializationAsFailedErrorResponseBody>> {
    let result = feature
        .mark_template_initialization_as_failed(request_body.template_hash)
        .await;

    match result {
        Ok(()) => Ok(Json(EmptyDto {})),
        Err(MarkTemplateInitializationAsFailedErrorResult::TemplateIsInitialized) => Err(Json(
            MarkTemplateInitializationAsFailedErrorResponseBody::TemplateIsInitialized {},
        )),
        Err(MarkTemplateInitializationAsFailedErrorResult::TemplateWasNotFound) => Err(Json(
            MarkTemplateInitializationAsFailedErrorResponseBody::TemplateWasNotFound {},
        )),
    }
}
