use std::time::Instant;

use axum::{extract::Request, middleware::Next, response::Response};
use tracing::{Instrument, Level, debug, error, span, warn};

pub async fn custom_trace_layer(request: Request, next: Next) -> Response {
    let start = Instant::now();

    let request_method = request.method().clone();
    let request_path = request.uri().path().to_owned();

    let span = span!(
        Level::DEBUG,
        "http_request",
        "{request_method} {request_path}"
    );

    async {
        let response = next.run(request).await;
        let elapsed = start.elapsed();

        let status = response.status();

        if status.is_success() {
            debug!(
                "{request_method} {request_path} Processed with {status} in {} ms",
                elapsed.as_millis()
            );
        } else if status.is_client_error() {
            warn!(
                "{request_method} {request_path} Processed with {status} in {} ms",
                elapsed.as_millis()
            );
        } else {
            error!(
                "{request_method} {request_path} Processed with {status} in {} ms",
                elapsed.as_millis()
            );
        };

        response
    }
    .instrument(span)
    .await
}
