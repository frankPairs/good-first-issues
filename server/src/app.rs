use std::{sync::Arc, time::Duration};

use axum::Router;
use bb8_redis::RedisConnectionManager;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    config::Settings, github::router::GithubRepositoryRouter,
    health_check::router::HealthCheckRouter,
    programing_languages::router::ProgrammingLanguageRouter, state::AppState,
};

const REDIS_POOL_CONNECTION_TIMEOUT: u64 = 10;

pub struct App {
    pub router: Router,
}

impl App {
    pub async fn new(settings: Settings) -> Result<App, anyhow::Error> {
        let github_settings = settings.github.clone();
        let redis_settings = settings.redis.clone();

        let redis_manager = RedisConnectionManager::new(redis_settings.url).unwrap();
        let redis_pool = bb8::Pool::builder()
            .connection_timeout(Duration::from_secs(REDIS_POOL_CONNECTION_TIMEOUT))
            .build(redis_manager)
            .await
            .expect("Failed to create Redis connection pool");

        let state = Arc::new(AppState {
            github_settings,
            redis_pool,
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
            .layer(CorsLayer::new().allow_origin(Any));

        Ok(App { router })
    }
}
