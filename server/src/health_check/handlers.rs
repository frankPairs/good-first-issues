use axum::response::Response;
use axum::{http::StatusCode, response::IntoResponse};

use crate::errors::GoodFirstIssuesError;

#[tracing::instrument(name = "Health check handler")]
pub async fn health_check() -> Result<Response, GoodFirstIssuesError> {
    return Ok((StatusCode::OK).into_response());
}
