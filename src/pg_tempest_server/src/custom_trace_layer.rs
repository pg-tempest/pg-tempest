use std::time::Instant;

use axum::{extract::Request, middleware::Next, response::Response};
use tracing::{Instrument, Level, error, info, span, warn};

pub async fn custom_trace_layer(request: Request, next: Next) -> Response {
    let start = Instant::now();

    let request_method = request.method();
    let request_path = request.uri().path();

    let span = span!(
        Level::INFO,
        "HTTP request",
        "{request_method} {request_path}"
    );

    async {
        let response = next.run(request).await;
        let elapsed = start.elapsed();

        let status = response.status();

        if status.is_success() {
            info!("Processed with {status} in {} ms", elapsed.as_millis());
        } else if status.is_client_error() {
            warn!("Processed with {status} in {} ms", elapsed.as_millis());
        } else {
            error!("Processed with {status} in {} ms", elapsed.as_millis());
        };

        response
    }
    .instrument(span)
    .await
}
