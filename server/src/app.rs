use std::sync::Arc;

use anyhow::Context;
use axum::{middleware, routing, Router};
use chrono::Duration;
use moka::future::Cache;
use tower::limit::ConcurrencyLimitLayer;
use tower_http::cors::{Any, CorsLayer};

use crate::{cache, config::Settings, github, health_check, programing_languages, state::AppState};

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

        let github_router = Router::new()
            .route(
                "/repositories",
                routing::get(github::handlers::get_repositories).layer(
                    middleware::from_fn_with_state(
                        state.clone(),
                        cache::middlewares::cache_middleware,
                    ),
                ),
            )
            .route(
                "/repositories/:repo/good-first-issues",
                routing::get(github::handlers::get_repository_good_first_issues).layer(
                    middleware::from_fn_with_state(
                        state.clone(),
                        cache::middlewares::cache_middleware,
                    ),
                ),
            )
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                github::middlewares::rate_limit_middleware,
            ));

        let router = Router::new()
            .route(
                "/healthcheck",
                routing::get(health_check::handlers::health_check),
            )
            .route(
                "/api/v1/programming-languages",
                routing::get(programing_languages::handlers::get_programming_languages),
            )
            .nest("/api/v1/github", github_router)
            .with_state(state.clone())
            .layer(ConcurrencyLimitLayer::new(MAX_CONCURRENCY_LIMIT))
            .layer(CorsLayer::new().allow_origin(Any));

        Ok(App { router })
    }
}
