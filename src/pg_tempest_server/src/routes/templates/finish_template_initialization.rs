use std::sync::Arc;

use axum::{Json, extract::State};
use pg_tempest_core::{
    features::templates::{
        TemplatesFeature, finish_template_initialization::FinishTemplateInitializationErrorResult,
    },
    models::value_types::template_hash::TemplateHash,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishTemplateInitializationRequestBody {
    template_hash: TemplateHash,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishTemplateInitializationOkResponseBody {}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FinishTemplateInitializationErrorResponseBody {
    TemplateNotFound {},
    TemplateInitializationWasFailed {},
}

pub async fn finish_template_initialization(
    State(feature): State<Arc<TemplatesFeature>>,
    Json(request_body): Json<FinishTemplateInitializationRequestBody>,
) -> Result<
    Json<FinishTemplateInitializationOkResponseBody>,
    Json<FinishTemplateInitializationErrorResponseBody>,
> {
    let result = feature
        .finish_template_initialization(request_body.template_hash)
        .await;

    match result {
        Ok(()) => Ok(Json(FinishTemplateInitializationOkResponseBody {})),
        Err(FinishTemplateInitializationErrorResult::TemplateInitializationWasFailed) => Err(Json(
            FinishTemplateInitializationErrorResponseBody::TemplateInitializationWasFailed {},
        )),
        Err(FinishTemplateInitializationErrorResult::TemplateNotFound) => Err(Json(
            FinishTemplateInitializationErrorResponseBody::TemplateNotFound {},
        )),
    }
}
