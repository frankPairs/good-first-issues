use std::sync::Arc;

use anyhow::Context;
use axum::Router;
use chrono::Duration;
use moka::future::Cache;
use tower::limit::ConcurrencyLimitLayer;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    config::Settings, github::router::GithubRepositoryRouter,
    health_check::router::HealthCheckRouter,
    programing_languages::router::ProgrammingLanguageRouter, state::AppState,
};

const MAX_CONCURRENCY_LIMIT: usize = 100;
const MAX_CACHE_CAPACITY: u64 = 10_000;
const CACHE_TIME_TO_LIVE: i64 = 600;

pub struct App {
    pub router: Router,
}

impl App {
    pub async fn new(settings: Settings) -> Result<App, anyhow::Error> {
        let github_settings = settings.github.clone();

        let time_to_live = Duration::seconds(CACHE_TIME_TO_LIVE)
            .to_std()
            .context("Invalid time to live value")?;
        let cache = Cache::builder()
            .max_capacity(MAX_CACHE_CAPACITY)
            .time_to_live(time_to_live)
            .build();

        let state = Arc::new(AppState {
            github_settings,
            cache,
        });
        let router = Router::new()
            .nest("/", HealthCheckRouter::build())
            .nest(
                "/api/v1/github",
                GithubRepositoryRouter::build(state.clone()),
            )
            .nest(
                "/api/v1/programming-languages",
                ProgrammingLanguageRouter::build(),
            )
            .with_state(state.clone())
            .layer(ConcurrencyLimitLayer::new(MAX_CONCURRENCY_LIMIT))
            .layer(CorsLayer::new().allow_origin(Any));

        Ok(App { router })
    }
}
