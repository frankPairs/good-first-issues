use reqwest::{header, Client, Url};

use crate::{config::GithubSettings, errors::RustGoodFirstIssuesError};

use super::models::{
    GetGithubRepositoriesParams, GetGithubRepositoriesResponse,
    GetGithubRepositoryGoodFirstIssuesParams, GetGithubRepositoryGoodFirstIssuesPathParams,
    GetGithubRepositoryGoodFirstIssuesResponse, GithubIssue, GithubIssueAPI, GithubPullRequest,
    GithubRepository as GithubRepositoryModel, SearchGithubRepositoriesResponseAPI,
};

const GITHUB_API_VERSION: &str = "2022-11-28";
const GITHUB_API_USERNAME: &str = "frankPairs";
const DEFAULT_PER_PAGE: u32 = 10;
const DEFAULT_PAGE: u32 = 1;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct GithubApiErrorPayload {
    message: String,
}

pub struct GithubHttpClient {
    http_client: Client,
    base_url: Url,
}

impl GithubHttpClient {
    pub fn new(settings: GithubSettings) -> Result<Self, RustGoodFirstIssuesError> {
        let mut headers = header::HeaderMap::new();

        headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
        headers.insert(
            "Authorization",
            format!("Bearer {}", settings.get_token()).parse().unwrap(),
        );
        headers.insert("X-GitHub-Api-Version", GITHUB_API_VERSION.parse().unwrap());
        headers.insert("User-Agent", GITHUB_API_USERNAME.parse().unwrap());

        let http_client: Client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(RustGoodFirstIssuesError::Reqwest)?;

        let base_url =
            Url::parse(&settings.get_api_url()).map_err(RustGoodFirstIssuesError::ParseUrl)?;

        Ok(Self {
            http_client,
            base_url,
        })
    }

    #[tracing::instrument(name = "Getrepositories from Github API", skip(self))]
    pub async fn get_repositories(
        &self,
        params: &GetGithubRepositoriesParams,
    ) -> Result<GetGithubRepositoriesResponse, RustGoodFirstIssuesError> {
        let mut url = self
            .base_url
            .join("/search/repositories?")
            .map_err(RustGoodFirstIssuesError::ParseUrl)?;

        url.query_pairs_mut()
            .append_pair(
                "q",
                format!("language:{} good-first-issues:>1", params.language).as_str(),
            )
            .append_pair("sort", "help-wanted-issues")
            .append_pair("order", "desc")
            .append_pair(
                "per_page",
                &params.per_page.unwrap_or(DEFAULT_PER_PAGE).to_string(),
            )
            .append_pair("page", &params.page.unwrap_or(DEFAULT_PAGE).to_string());

        let response = self
            .http_client
            .get(url)
            .send()
            .await
            .map_err(RustGoodFirstIssuesError::Reqwest)?;

        if !response.status().is_success() {
            return Err(self.parse_error_from_response(response).await);
        }

        let json: SearchGithubRepositoriesResponseAPI = response
            .json()
            .await
            .map_err(RustGoodFirstIssuesError::Reqwest)?;

        Ok(GetGithubRepositoriesResponse {
            total_count: json.total_count,
            items: json
                .items
                .into_iter()
                .map(|repo| GithubRepositoryModel {
                    id: repo.id,
                    url: repo.html_url,
                    name: repo.full_name,
                    private: repo.private,
                    avatar_url: repo.owner.avatar_url,
                    description: repo.description,
                    stars_count: repo.stargazers_count,
                    open_issues_count: repo.open_issues_count,
                    has_issues: repo.has_issues,
                    license: match repo.license {
                        Some(license) => Some(license.name),
                        None => None,
                    },
                })
                .collect(),
        })
    }

    #[tracing::instrument(name = "Get good first issues from a Github repository", skip(self))]
    pub async fn get_repository_good_first_issues(
        &self,
        path_params: &GetGithubRepositoryGoodFirstIssuesPathParams,
        params: &GetGithubRepositoryGoodFirstIssuesParams,
    ) -> Result<GetGithubRepositoryGoodFirstIssuesResponse, RustGoodFirstIssuesError> {
        let mut url = self
            .base_url
            .join(&format!(
                "/repos/{}/{}/issues?",
                params.owner, path_params.repo
            ))
            .map_err(RustGoodFirstIssuesError::ParseUrl)?;

        url.query_pairs_mut()
            .append_pair("labels", "good first issue")
            .append_pair("sort", "updated")
            .append_pair("direction", "desc")
            .append_pair(
                "per_page",
                &params.per_page.unwrap_or(DEFAULT_PER_PAGE).to_string(),
            )
            .append_pair("page", &params.page.unwrap_or(DEFAULT_PAGE).to_string());

        let response = self
            .http_client
            .get(url)
            .send()
            .await
            .map_err(RustGoodFirstIssuesError::Reqwest)?;

        if !response.status().is_success() {
            return Err(self.parse_error_from_response(response).await);
        }

        let json: Vec<GithubIssueAPI> = response
            .json()
            .await
            .map_err(RustGoodFirstIssuesError::Reqwest)?;

        Ok(GetGithubRepositoryGoodFirstIssuesResponse {
            items: json
                .into_iter()
                .map(|issue| GithubIssue {
                    id: issue.id,
                    body: issue.body,
                    description: issue.description,
                    state: issue.state,
                    title: issue.title,
                    url: issue.html_url,
                    pull_request: if let Some(pull_request) = issue.pull_request {
                        Some(GithubPullRequest {
                            url: pull_request.html_url,
                        })
                    } else {
                        None
                    },
                })
                .collect(),
        })
    }

    async fn parse_error_from_response(
        &self,
        response: reqwest::Response,
    ) -> RustGoodFirstIssuesError {
        let status_code = response.status();
        let headers = response.headers().clone();
        let result: Result<GithubApiErrorPayload, reqwest::Error> = response.json().await;

        match result {
            Ok(error_payload) => {
                RustGoodFirstIssuesError::GithubAPI(status_code, headers, error_payload.message)
            }
            Err(err) => RustGoodFirstIssuesError::Reqwest(err),
        }
    }
}
