use std::{str::FromStr, sync::Arc};

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
    template_hash: Box<str>,
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
    let template_hash = TemplateHash::from_str(&request_body.template_hash)
        .map_err(|e| e.to_string())
        .unwrap();

    let result = feature.finish_template_initialization(template_hash).await;

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
