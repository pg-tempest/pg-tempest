use std::{sync::Arc, time::Duration};

use crate::dtos::{db_connection_options_dto::DbConnectionOptionsDto, json_response::JsonResponse};
use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use pg_tempest_core::models::value_types::pg_identifier::PgIdentifier;
use pg_tempest_core::{
    PgTempestCore,
    features::templates::start_template_initialization::StartTemplateInitializationResult,
    models::value_types::template_hash::TemplateHash,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartTemplateInitializationRequestBody {
    template_hash: TemplateHash,
    initialization_duration_ms: u64,
    parent_template_db_name: Option<PgIdentifier>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum StartTemplateInitializationResponseBody {
    InitializationWasStarted {
        database_connection_options: DbConnectionOptionsDto,
        initialization_deadline: DateTime<Utc>,
    },
    InitializationIsInProgress {},
    InitializationIsFinished {},
    InitializationIsFailed {},
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
            Duration::from_millis(request_body.initialization_duration_ms),
            request_body.parent_template_db_name,
        )
        .await;

    match result {
        Ok(StartTemplateInitializationResult::InitializationWasStarted {
            database_connection_options,
            initialization_deadline,
        }) => JsonResponse {
            status_code: StatusCode::OK,
            body: StartTemplateInitializationResponseBody::InitializationWasStarted {
                database_connection_options: database_connection_options.into(),
                initialization_deadline,
            },
        },
        Ok(StartTemplateInitializationResult::InitializationIsInProgress) => JsonResponse {
            status_code: StatusCode::OK,
            body: StartTemplateInitializationResponseBody::InitializationIsInProgress {},
        },
        Ok(StartTemplateInitializationResult::InitializationIsFinished) => JsonResponse {
            status_code: StatusCode::OK,
            body: StartTemplateInitializationResponseBody::InitializationIsFinished {},
        },
        Ok(StartTemplateInitializationResult::InitializationIsFailed) => JsonResponse {
            status_code: StatusCode::OK,
            body: StartTemplateInitializationResponseBody::InitializationIsFailed {},
        },
        Err(err) => JsonResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            body: StartTemplateInitializationResponseBody::UnexpectedError {
                message: err.to_string().into(),
            },
        },
    }
}
