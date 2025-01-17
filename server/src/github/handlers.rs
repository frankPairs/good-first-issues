use axum::extract::Path;
use axum::response::Response;
use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::errors::RustGoodFirstIssuesError;
use crate::github::models::GetGithubRepositoriesParams;
use crate::state::AppState;

use super::client::GithubHttpClient;
use super::models::{
    GetGithubRepositoryGoodFirstIssuesParams, GetGithubRepositoryGoodFirstIssuesPathParams,
};

#[tracing::instrument(name = "Get Github repositories handler", skip(state))]
pub async fn get_repositories(
    state: State<Arc<AppState>>,
    params: Query<GetGithubRepositoriesParams>,
) -> Result<Response, RustGoodFirstIssuesError> {
    let params = params.0;
    let github_client = GithubHttpClient::new(state.github_settings.clone())?;

    let res = github_client.get_rust_repositories(&params).await?;

    return Ok((StatusCode::OK, Json(res)).into_response());
}

#[tracing::instrument(name = "Get Github repository good first issues", skip(state))]
pub async fn get_repository_good_first_issues(
    state: State<Arc<AppState>>,
    path: Path<GetGithubRepositoryGoodFirstIssuesPathParams>,
    params: Query<GetGithubRepositoryGoodFirstIssuesParams>,
) -> Result<Response, RustGoodFirstIssuesError> {
    let params = params.0;
    let path_params = path.0;

    let github_client = GithubHttpClient::new(state.github_settings.clone())?;

    let res = github_client
        .get_repository_good_first_issues(&path_params, &params)
        .await?;

    return Ok((StatusCode::OK, Json(res)).into_response());
}
