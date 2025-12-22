use axum::response::Response;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};

use crate::errors::GoodFirstIssuesError;

use super::repositories::ProgramingLanguageRepository;

#[tracing::instrument(name = "Get Programming Languages handler")]
pub async fn get_programming_languages() -> Result<Response, GoodFirstIssuesError> {
    let repo = ProgramingLanguageRepository::new();

    let res = repo.get();

    return Ok((StatusCode::OK, Json(res)).into_response());
}
