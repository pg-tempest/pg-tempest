use std::{sync::Arc, time::Duration};

use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use pg_tempest_core::{
    PgTempestCore,
    features::templates::start_template_initialization::StartTemplateInitializationOkResult,
    models::value_types::template_hash::TemplateHash,
};
use serde::{Deserialize, Serialize};

use crate::dtos::{db_connection_options_dto::DbConnectionOptionsDto, json_response::JsonResponse};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartTemplateInitializationRequestBody {
    template_hash: TemplateHash,
    initialization_duration_in_seconds: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum StartTemplateInitializationResponseBody {
    InitializationWasStarted {
        database_connection_options: DbConnectionOptionsDto,
        initialization_deadline: DateTime<Utc>,
    },
    InitializationIsInProgress {
        initialization_deadline: DateTime<Utc>,
    },
    InitializationIsFinished {},
    UnexpectedError {
        message: Box<str>,
    },
}

pub async fn start_template_initialization(
    State(tempest_core): State<Arc<PgTempestCore>>,
    Json(request_body): Json<StartTemplateInitializationRequestBody>,
) -> JsonResponse<StartTemplateInitializationResponseBody> {
    let result = tempest_core
        .start_template_initialization(
            request_body.template_hash,
            Duration::from_secs(request_body.initialization_duration_in_seconds),
        )
        .await;

    match result {
        Ok(StartTemplateInitializationOkResult::InitializationWasStarted {
            database_connection_options,
            initialization_deadline,
        }) => JsonResponse {
            status_code: StatusCode::OK,
            body: StartTemplateInitializationResponseBody::InitializationWasStarted {
                database_connection_options: database_connection_options.into(),
                initialization_deadline,
            },
        },
        Ok(StartTemplateInitializationOkResult::InitializationIsInProgress {
            initialization_deadline,
        }) => JsonResponse {
            status_code: StatusCode::OK,
            body: StartTemplateInitializationResponseBody::InitializationIsInProgress {
                initialization_deadline,
            },
        },
        Ok(StartTemplateInitializationOkResult::InitializationIsFinished) => JsonResponse {
            status_code: StatusCode::OK,
            body: StartTemplateInitializationResponseBody::InitializationIsFinished {},
        },
        Err(err) => JsonResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            body: StartTemplateInitializationResponseBody::UnexpectedError {
                message: err.to_string().into(),
            },
        },
    }
}
