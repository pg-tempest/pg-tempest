use std::{sync::Arc, time::Duration};

use axum::{Json, extract::State};
use chrono::{DateTime, Utc};
use pg_tempest_core::{
    features::templates::{
        TemplatesFeature, extend_template_initialization::ExtendTemplateInitializationErrorResult,
    },
    models::value_types::template_hash::TemplateHash,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendTemplateInitializationRequestBody {
    template_hash: TemplateHash,
    additional_time: Duration,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendTemplateInitializationOkResponseBody {
    new_initialization_deadline: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ExtendTemplateInitializationErrorResponseBody {
    TemplateWasNotFound {},
    TemplateIsInitialized {},
    TemplateInitializationWasFailed {},
}

pub async fn extend_template_initialization(
    State(feature): State<Arc<TemplatesFeature>>,
    Json(request_body): Json<ExtendTemplateInitializationRequestBody>,
) -> Result<
    Json<ExtendTemplateInitializationOkResponseBody>,
    Json<ExtendTemplateInitializationErrorResponseBody>,
> {
    let result = feature
        .extend_template_initialization(request_body.template_hash, request_body.additional_time)
        .await;

    match result {
        Ok(result) => Ok(Json(ExtendTemplateInitializationOkResponseBody {
            new_initialization_deadline: result.new_initialization_deadline,
        })),
        Err(ExtendTemplateInitializationErrorResult::TemplateWasNotFound) => Err(Json(
            ExtendTemplateInitializationErrorResponseBody::TemplateWasNotFound {},
        )),
        Err(ExtendTemplateInitializationErrorResult::TemplateIsInitialized) => Err(Json(
            ExtendTemplateInitializationErrorResponseBody::TemplateIsInitialized {},
        )),
        Err(ExtendTemplateInitializationErrorResult::TemplateInitializationWasFailed) => Err(Json(
            ExtendTemplateInitializationErrorResponseBody::TemplateInitializationWasFailed {},
        )),
    }
}
