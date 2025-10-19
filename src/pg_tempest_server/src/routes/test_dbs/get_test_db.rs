use std::{sync::Arc, time::Duration};

use axum::{Json, extract::State};
use pg_tempest_core::{
    features::test_dbs::{TestDbsFeature, get_test_db::GetTestDbErrorResult},
    models::value_types::{template_hash::TemplateHash, test_db_id::TestDbId},
};
use serde::{Deserialize, Serialize};

use crate::dtos::db_connection_options_dto::DbConnectionOptionsDto;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTestDbRequestBody {
    pub template_hash: TemplateHash,
    pub usage_duration: Duration,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTestDbOkResponse {
    pub test_db_id: TestDbId,
    pub db_connection_options: DbConnectionOptionsDto,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetTestDbErrorResponse {
    TemplateWasNotFound {},
    TemplateIsNotInitialized {},
    Unknown { inner: Box<str> },
}

pub async fn get_test_db(
    State(feature): State<Arc<TestDbsFeature>>,
    Json(request_body): Json<GetTestDbRequestBody>,
) -> Result<Json<GetTestDbOkResponse>, Json<GetTestDbErrorResponse>> {
    let result = feature
        .get_test_db(request_body.template_hash, request_body.usage_duration)
        .await;

    match result {
        Ok(result) => Ok(Json(GetTestDbOkResponse {
            test_db_id: result.test_db_id,
            db_connection_options: result.connection_options.into(),
        })),
        Err(GetTestDbErrorResult::TemplateWasNotFound) => {
            Err(Json(GetTestDbErrorResponse::TemplateWasNotFound {}))
        }
        Err(GetTestDbErrorResult::TemplateIsNotInitalized) => {
            Err(Json(GetTestDbErrorResponse::TemplateIsNotInitialized {}))
        }
        Err(GetTestDbErrorResult::Unknown { inner }) => {
            Err(Json(GetTestDbErrorResponse::Unknown {
                inner: inner.to_string().into(),
            }))
        }
    }
}
