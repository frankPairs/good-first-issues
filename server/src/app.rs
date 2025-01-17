use std::{sync::Arc, time::Duration};

use axum::Router;
use bb8_redis::RedisConnectionManager;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    config::Settings, github::router::GithubRepositoryRouter,
    health_check::router::HealthCheckRouter, state::AppState,
};

const REDIS_POOL_CONNECTION_TIMEOUT: u64 = 10;

pub struct App {
    pub router: Router,
    #[allow(dead_code)]
    pub state: Arc<AppState>,
}

impl App {
    pub async fn new(settings: Settings) -> Result<App, anyhow::Error> {
        let github_settings = settings.github.clone();
        let redis_settings = settings.redis.clone();

        let redis_manager = RedisConnectionManager::new(redis_settings.url).unwrap();
        let redis_pool = bb8::Pool::builder()
            .connection_timeout(Duration::from_secs(REDIS_POOL_CONNECTION_TIMEOUT))
            .build(redis_manager)
            .await?;

        let state = Arc::new(AppState {
            github_settings,
            redis_pool,
        });
        let router = Router::new()
            .nest("/", HealthCheckRouter::build())
            .layer(CorsLayer::new().allow_origin(Any))
            .nest(
                "/api/v1/github",
                GithubRepositoryRouter::build(state.clone()),
            )
            .with_state(state.clone());

        Ok(App { router, state })
    }
}
