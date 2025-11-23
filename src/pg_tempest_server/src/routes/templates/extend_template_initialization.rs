use std::{sync::Arc, time::Duration};

use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use pg_tempest_core::{
    PgTempestCore,
    features::templates::extend_template_initialization::ExtendTemplateInitializationErrorResult,
    models::value_types::template_hash::TemplateHash,
};
use serde::{Deserialize, Serialize};

use crate::dtos::json_response::JsonResponse;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendTemplateInitializationRequestBody {
    template_hash: TemplateHash,
    additional_time_ms: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ExtendTemplateInitializationResponseBody {
    InitializationWasExtended {
        new_initialization_deadline: DateTime<Utc>,
    },
    TemplateWasNotFound {},
    InitializationIsNotStarted {},
    InitializationIsFinished {},
    InitializationIsFailed {
        reason: Option<Arc<str>>,
    },
}

pub async fn extend_template_initialization(
    State(tempest_core): State<Arc<PgTempestCore>>,
    Json(request_body): Json<ExtendTemplateInitializationRequestBody>,
) -> JsonResponse<ExtendTemplateInitializationResponseBody> {
    let result = tempest_core
        .extend_template_initialization(
            request_body.template_hash,
            Duration::from_millis(request_body.additional_time_ms),
        )
        .await;

    match result {
        Ok(result) => JsonResponse {
            status_code: StatusCode::OK,
            body: ExtendTemplateInitializationResponseBody::InitializationWasExtended {
                new_initialization_deadline: result.new_initialization_deadline,
            },
        },
        Err(ExtendTemplateInitializationErrorResult::TemplateWasNotFound) => JsonResponse {
            status_code: StatusCode::NOT_FOUND,
            body: ExtendTemplateInitializationResponseBody::TemplateWasNotFound {},
        },
        Err(ExtendTemplateInitializationErrorResult::InitializationIsFinished) => JsonResponse {
            status_code: StatusCode::CONFLICT,
            body: ExtendTemplateInitializationResponseBody::InitializationIsFinished {},
        },
        Err(ExtendTemplateInitializationErrorResult::InitializationIsFailed { reason }) => {
            JsonResponse {
                status_code: StatusCode::CONFLICT,
                body: ExtendTemplateInitializationResponseBody::InitializationIsFailed { reason },
            }
        }
        Err(ExtendTemplateInitializationErrorResult::InitializationIsNotStarted) => JsonResponse {
            status_code: StatusCode::CONFLICT,
            body: ExtendTemplateInitializationResponseBody::InitializationIsNotStarted {},
        },
    }
}
