use axum::response::Response;
use axum::{http::StatusCode, response::IntoResponse};

use crate::errors::RustGoodFirstIssuesError;

#[tracing::instrument(name = "Health check handler")]
pub async fn health_check() -> Result<Response, RustGoodFirstIssuesError> {
    return Ok((StatusCode::OK).into_response());
}
