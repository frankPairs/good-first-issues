use std::sync::Arc;

use axum::{routing, Router};

use crate::state::AppState;

use super::handlers::get_programming_languages;

pub struct ProgrammingLanguageRouter;

impl ProgrammingLanguageRouter {
    pub fn build() -> Router<Arc<AppState>> {
        Router::new().route("/", routing::get(get_programming_languages))
    }
}
