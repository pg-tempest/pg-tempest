use std::sync::Arc;

use axum::{Json, extract::State};
use chrono::{DateTime, TimeDelta, Utc};
use pg_tempest_core::{
    features::templates::{
        TemplatesFeature, start_template_initialization::StartTemplateInitializationOkResult,
    },
    models::value_types::template_hash::TemplateHash,
};
use serde::{Deserialize, Serialize};

use crate::dtos::db_connection_options_dto::DbConnectionOptionsDto;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartTemplateInitializationRequestBody {
    template_hash: TemplateHash,
    initialization_duration: TimeDelta,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum StartTemplateInitializationResponseBody {
    Started {
        database_connection_options: DbConnectionOptionsDto,
        initialization_deadline: DateTime<Utc>,
    },
    InProgress {
        initialization_deadline: DateTime<Utc>,
    },
    Initialized {},
}

pub async fn start_template_initialization(
    State(feature): State<Arc<TemplatesFeature>>,
    Json(request_body): Json<StartTemplateInitializationRequestBody>,
) -> Result<Json<StartTemplateInitializationResponseBody>, String> {
    let result = feature
        .start_template_initialization(
            request_body.template_hash,
            request_body.initialization_duration,
        )
        .await
        .map_err(|e| e.to_string())?;

    let response_body = match result {
        StartTemplateInitializationOkResult::InProgress {
            initialization_deadline,
        } => StartTemplateInitializationResponseBody::InProgress {
            initialization_deadline,
        },
        StartTemplateInitializationOkResult::Started {
            database_connection_options,
            initialization_deadline,
        } => StartTemplateInitializationResponseBody::Started {
            database_connection_options: database_connection_options.into(),
            initialization_deadline,
        },
        StartTemplateInitializationOkResult::Initialized => {
            StartTemplateInitializationResponseBody::Initialized {}
        }
    };

    Ok(Json(response_body))
}
