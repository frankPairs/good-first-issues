use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use thiserror::Error;

const GITHUB_RATE_LIMIT_HEADERS: [&str; 3] =
    ["retry-after", "x-ratelimit-remaining", "x-ratelimit-reset"];

#[derive(Debug, Error)]
pub enum GoodFirstIssuesError {
    #[error("Reqwest Error = {0}")]
    Reqwest(reqwest::Error),
    #[error("GithubAPI Error = {0}:{2}")]
    GithubAPI(StatusCode, HeaderMap<HeaderValue>, String),
    #[error("ParseUrl Error = {0}")]
    ParseUrl(url::ParseError),
    #[error("Axum Error = {0}")]
    Axum(axum::Error),
    #[error("Cache error = {0}")]
    Cache(String),
}

impl IntoResponse for GoodFirstIssuesError {
    fn into_response(self) -> Response {
        let err_message = self.to_string();

        tracing::error!("{}", err_message);

        match self {
            GoodFirstIssuesError::GithubAPI(status_code, headers, _) => {
                // It just returns the rate limit headers. This is because the other headers from Github are not necessary
                // on this project.
                let rate_limit_headers = HeaderMap::from_iter(
                    headers
                        .iter()
                        .filter(|(name, _)| GITHUB_RATE_LIMIT_HEADERS.contains(&name.as_str()))
                        .map(|(name, value)| (name.clone(), value.clone())),
                );

                // Just returning the rate limit headers from Github API
                (status_code, rate_limit_headers, err_message).into_response()
            }
            GoodFirstIssuesError::Reqwest(err) => (
                err.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                err_message,
            )
                .into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, err_message).into_response(),
        }
    }
}
