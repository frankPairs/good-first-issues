use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use std::error::Error;

const GITHUB_RATE_LIMIT_HEADERS: [&str; 3] =
    ["retry-after", "x-ratelimit-remaining", "x-ratelimit-reset"];

#[derive(Debug)]
pub enum GoodFirstIssuesError {
    Reqwest(reqwest::Error),
    GithubAPI(StatusCode, HeaderMap<HeaderValue>, String),
    ParseUrl(url::ParseError),
    Axum(axum::Error),
    Unknown(String),
}

impl std::fmt::Display for GoodFirstIssuesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GoodFirstIssuesError::Reqwest(err) => {
                tracing::error!("ReqwestError url = {:?}", err.url());
                tracing::error!("ReqwestError status = {:?}", err.status());
                tracing::error!("ReqwestError source = {:?}", err.source());

                write!(f, "ReqwestError error: {}", err)
            }
            GoodFirstIssuesError::ParseUrl(err) => {
                write!(f, "Parse url error: {}", err)
            }
            GoodFirstIssuesError::GithubAPI(status_code, _, message) => {
                write!(f, "Github API error {}: {}", status_code, message)
            }
            GoodFirstIssuesError::Axum(err) => {
                let error_msg = format!("Axum internal error: {}", err);

                write!(f, "{}", error_msg)
            }
            GoodFirstIssuesError::Unknown(err) => {
                let error_msg = format!("Unknown error : {}", err);

                write!(f, "{}", error_msg)
            }
        }
    }
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
