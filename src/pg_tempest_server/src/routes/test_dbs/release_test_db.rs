use std::sync::Arc;

use axum::{Json, extract::State};
use pg_tempest_core::{
    features::test_dbs::{TestDbsFeature, release_test_db::ReleaseTestDbErrorResult},
    models::value_types::{template_hash::TemplateHash, test_db_id::TestDbId},
};
use serde::{Deserialize, Serialize};

use crate::dtos::empty_dto::EmptyDto;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseTestDbRequestBody {
    template_hash: TemplateHash,
    test_db_id: TestDbId,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ReleaseTestDbErrorResponse {
    TemplateWasNotFound {},
    TestDbWasNotFound {},
}

pub async fn release_test_db(
    State(feature): State<Arc<TestDbsFeature>>,
    Json(request_body): Json<ReleaseTestDbRequestBody>,
) -> Result<Json<EmptyDto>, Json<ReleaseTestDbErrorResponse>> {
    let result = feature
        .release_test_db(request_body.template_hash, request_body.test_db_id)
        .await;

    match result {
        Ok(_) => Ok(Json(EmptyDto {})),
        Err(ReleaseTestDbErrorResult::TemplateWasNotFound) => {
            Err(Json(ReleaseTestDbErrorResponse::TemplateWasNotFound {}))
        }
        Err(ReleaseTestDbErrorResult::TestDbWasNotFound) => {
            Err(Json(ReleaseTestDbErrorResponse::TestDbWasNotFound {}))
        }
    }
}
