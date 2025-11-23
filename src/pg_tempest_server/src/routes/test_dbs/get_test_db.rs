use std::{sync::Arc, time::Duration};

use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use pg_tempest_core::{
    PgTempestCore,
    features::test_dbs::get_test_db::GetTestDbErrorResult,
    models::value_types::{template_hash::TemplateHash, test_db_id::TestDbId},
};
use serde::{Deserialize, Serialize};

use crate::dtos::{db_connection_options_dto::DbConnectionOptionsDto, json_response::JsonResponse};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTestDbRequestBody {
    pub template_hash: TemplateHash,
    pub usage_duration_ms: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum GetTestDbResponseBody {
    TestDbWasCreated {
        test_db_id: TestDbId,
        db_connection_options: DbConnectionOptionsDto,
        usage_deadline: DateTime<Utc>,
    },
    TemplateWasNotFound {},
    TemplateIsNotInitialized {},
    UnknownError {
        message: Box<str>,
    },
}

pub async fn get_test_db(
    State(tempest_core): State<Arc<PgTempestCore>>,
    Json(request_body): Json<GetTestDbRequestBody>,
) -> JsonResponse<GetTestDbResponseBody> {
    let result = tempest_core
        .get_test_db(
            request_body.template_hash,
            Duration::from_millis(request_body.usage_duration_ms),
        )
        .await;

    match result {
        Ok(result) => JsonResponse {
            status_code: StatusCode::OK,
            body: GetTestDbResponseBody::TestDbWasCreated {
                test_db_id: result.test_db_id,
                db_connection_options: result.connection_options.into(),
                usage_deadline: result.usage_deadline,
            },
        },
        Err(GetTestDbErrorResult::TemplateWasNotFound) => JsonResponse {
            status_code: StatusCode::NOT_FOUND,
            body: GetTestDbResponseBody::TemplateWasNotFound {},
        },
        Err(GetTestDbErrorResult::TemplateIsNotInitialized) => JsonResponse {
            status_code: StatusCode::CONFLICT,
            body: GetTestDbResponseBody::TemplateIsNotInitialized {},
        },
        Err(GetTestDbErrorResult::Unknown { inner }) => JsonResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            body: GetTestDbResponseBody::UnknownError {
                message: inner.to_string().into(),
            },
        },
    }
}
