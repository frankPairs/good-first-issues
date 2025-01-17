use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use bb8::PooledConnection;
use bb8_redis::RedisConnectionManager;
use http_body_util::BodyExt;
use redis::{AsyncCommands, FromRedisValue, JsonAsyncCommands};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

use crate::{errors::RustGoodFirstIssuesError, state::AppState};

use super::extractors::ExtractRedisKey;

const DEFAULT_REDIS_EXPIRATION_TIME: i64 = 600;
const DEFAULT_REDIS_DEFAULT_PATH: &str = "$";

pub async fn redis_cache_middleware<RedisCacheResponseValue>(
    State(state): State<Arc<AppState>>,
    ExtractRedisKey(redis_key): ExtractRedisKey,
    request: Request,
    next: Next,
) -> Result<Response, RustGoodFirstIssuesError>
where
    RedisCacheResponseValue: DeserializeOwned + FromRedisValue + Serialize + Debug + Send + Sync,
{
    let mut redis_conn = state
        .redis_pool
        .get()
        .await
        .map_err(RustGoodFirstIssuesError::RedisConnection)?;

    if redis_conn.exists(&redis_key).await.unwrap_or(false) {
        let redis_response_builder = RedisResponseBuilder::new(redis_conn, &redis_key);

        return redis_response_builder
            .build::<RedisCacheResponseValue>()
            .await;
    }

    let res: Response = next.run(request).await;
    let res_status: StatusCode = res.status();

    // If there is a response error, we return the response as we do not need to save on Redis.
    if res_status.is_client_error() || res_status.is_server_error() {
        return Ok(res);
    }

    // It builds the response from the handler and saves it to Redis before returning it.
    let handler_response_builder = HandlerResponseBuilder::new(redis_conn, &redis_key);

    handler_response_builder
        .build::<RedisCacheResponseValue>(res)
        .await
}

// Builds the middleware response based on the data coming from Redis cache
struct RedisResponseBuilder<'a> {
    redis_conn: PooledConnection<'a, RedisConnectionManager>,
    redis_key: &'a str,
}

impl<'a> RedisResponseBuilder<'a> {
    fn new(redis_conn: PooledConnection<'a, RedisConnectionManager>, redis_key: &'a str) -> Self {
        RedisResponseBuilder {
            redis_conn,
            redis_key,
        }
    }

    // Sets the Cache-Control header using the expiration time in seconds.
    fn set_cache_headers(
        &mut self,
        headers: &mut HeaderMap<HeaderValue>,
        expiration_time: Option<i64>,
    ) {
        // If the expiration time is less than or equal to zero, it means that the key exists but it does not contain
        // any expiration time. In this case, we do not set the Cache-Control header.
        let expiration_time: Option<i64> = expiration_time.filter(|time| *time > 0);

        if let Some(expiration_time) = expiration_time {
            headers.append(
                "Cache-Control",
                HeaderValue::from_str(&format!("max-age={}", expiration_time)).unwrap(),
            );
        }
    }

    async fn build<
        RedisCacheResponseValue: DeserializeOwned + FromRedisValue + Serialize + Debug + Send + Sync,
    >(
        mut self,
    ) -> Result<Response, RustGoodFirstIssuesError> {
        let res: RedisCacheResponseValue = self
            .redis_conn
            .json_get(self.redis_key, DEFAULT_REDIS_DEFAULT_PATH)
            .await
            .map_err(RustGoodFirstIssuesError::Redis)?;

        let mut headers: HeaderMap<HeaderValue> = HeaderMap::new();

        let expiration_time = self.redis_conn.ttl(self.redis_key).await.unwrap_or(None);

        self.set_cache_headers(&mut headers, expiration_time);

        Ok((StatusCode::OK, headers, Json(res)).into_response())
    }
}

// Builds the middleware response based on the data coming from a handler.
// It saves the response within redis before sending it back through the middleware chain.
struct HandlerResponseBuilder<'a> {
    redis_conn: PooledConnection<'a, RedisConnectionManager>,
    redis_key: &'a str,
}

impl<'a> HandlerResponseBuilder<'a> {
    fn new(redis_conn: PooledConnection<'a, RedisConnectionManager>, redis_key: &'a str) -> Self {
        HandlerResponseBuilder {
            redis_conn,
            redis_key,
        }
    }

    // Saves the response from the handler to Redis.
    async fn save_response_to_redis<
        RedisCacheResponseValue: DeserializeOwned + FromRedisValue + Serialize + Debug + Send + Sync,
    >(
        &mut self,
        key: &str,
        value: RedisCacheResponseValue,
        expiration_time: i64,
    ) -> Result<(), RustGoodFirstIssuesError> {
        // Save response to Redis
        self.redis_conn
            .json_set::<&str, &str, RedisCacheResponseValue, ()>(
                key,
                DEFAULT_REDIS_DEFAULT_PATH,
                &value,
            )
            .await
            .map_err(RustGoodFirstIssuesError::Redis)?;

        // Set expiration time using
        self.redis_conn
            .expire::<&str, ()>(key, expiration_time)
            .await
            .map_err(RustGoodFirstIssuesError::Redis)?;

        Ok(())
    }

    async fn build<
        RedisCacheResponseValue: DeserializeOwned + FromRedisValue + Serialize + Debug + Send + Sync,
    >(
        mut self,
        res: Response,
    ) -> Result<Response, RustGoodFirstIssuesError> {
        let (parts, body) = res.into_parts();

        let bytes = body
            .collect()
            .await
            .map_err(RustGoodFirstIssuesError::Axum)?;
        let bytes = bytes.to_bytes();

        let res_json_str =
            String::from_utf8(bytes.to_vec()).map_err(RustGoodFirstIssuesError::FromUtf8Error)?;

        let res_body: RedisCacheResponseValue =
            serde_json::from_str(&res_json_str).map_err(RustGoodFirstIssuesError::SerdeJson)?;

        self.save_response_to_redis(self.redis_key, res_body, DEFAULT_REDIS_EXPIRATION_TIME)
            .await?;

        Ok(Response::from_parts(parts, Body::from(bytes)))
    }
}
