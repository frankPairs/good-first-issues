use crate::{
    github::handlers::{get_repositories, get_repository_good_first_issues},
    state::AppState,
};
use axum::{handler::Handler, middleware, routing, Router};
use std::sync::Arc;

use super::middlewares::rate_limit_middleware;
use crate::cache::middlewares::cache_middleware;

pub struct GithubRepositoryRouter;

impl GithubRepositoryRouter {
    pub fn build(state: Arc<AppState>) -> Router<Arc<AppState>> {
        Router::new()
            .route(
                "/repositories",
                routing::get(get_repositories).layer(middleware::from_fn_with_state(
                    state.clone(),
                    cache_middleware,
                )),
            )
            .route(
                "/repositories/:repo/good-first-issues",
                routing::get(get_repository_good_first_issues.layer(
                    middleware::from_fn_with_state(state.clone(), cache_middleware),
                )),
            )
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                rate_limit_middleware,
            ))
            .with_state(state)
    }
}
