use chrono::{DateTime, Utc};
use moka::future::Cache;

use crate::config::GithubSettings;

#[derive(Clone, Debug)]
pub struct CacheValue {
    pub data: Vec<u8>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub github_settings: GithubSettings,
    pub cache: Cache<String, CacheValue>,
}
