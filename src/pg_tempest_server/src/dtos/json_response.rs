use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

pub struct JsonResponse<T: Serialize> {
    pub status_code: StatusCode,
    pub body: T,
}

impl<T: Serialize> IntoResponse for JsonResponse<T> {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, Json(self.body)).into_response()
    }
}
