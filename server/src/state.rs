use bb8::Pool;
use bb8_redis::RedisConnectionManager;

use crate::config::GithubSettings;

#[derive(Clone, Debug)]
pub struct AppState {
    pub github_settings: GithubSettings,
    pub redis_pool: Pool<RedisConnectionManager>,
}
