use std::sync::Arc;

use axum::{routing, Router};

use crate::state::AppState;

use super::handlers::health_check;

pub struct HealthCheckRouter;

impl HealthCheckRouter {
    pub fn build() -> Router<Arc<AppState>> {
        Router::new().route("/health-check", routing::get(health_check))
    }
}
