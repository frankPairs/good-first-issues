use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use chrono::Utc;
use http_body_util::BodyExt;

use crate::{
    errors::GoodFirstIssuesError,
    state::{AppState, CacheValue},
};

use super::extractors::ExtractCacheKey;

const CACHE_EXPIRATION_TIME: i64 = 600;

pub async fn cache_middleware(
    State(state): State<Arc<AppState>>,
    ExtractCacheKey(key): ExtractCacheKey,
    request: Request,
    next: Next,
) -> Result<Response, GoodFirstIssuesError> {
    match state.cache.get(&key).await {
        // Cache hit
        Some(v) => {
            let headers = get_cache_headers(Some(&v))?;

            Ok((StatusCode::OK, headers, v.data).into_response())
        }
        // Cache miss
        None => {
            let res: Response = next.run(request).await;
            let res_status: StatusCode = res.status();

            // If there is a response error, we return the response as we do not need to save on
            // the cache.
            if res_status.is_client_error() || res_status.is_server_error() {
                return Ok(res);
            }

            let headers = get_cache_headers(None)?;

            let (_, body) = res.into_parts();

            let bytes = body.collect().await.map_err(GoodFirstIssuesError::Axum)?;
            let bytes = bytes.to_bytes();

            // It stores the data coming from the handler into the cache
            state
                .cache
                .insert(
                    key,
                    CacheValue {
                        data: bytes.to_vec(),
                        last_modified: Utc::now(),
                    },
                )
                .await;

            Ok((res_status, headers, bytes).into_response())
        }
    }
}

fn get_cache_headers(
    value: Option<&CacheValue>,
) -> Result<HeaderMap<HeaderValue>, GoodFirstIssuesError> {
    let mut headers: HeaderMap<HeaderValue> = HeaderMap::new();

    match value {
        Some(v) => {
            let max_age = HeaderValue::from_str(&format!("max-age={}", CACHE_EXPIRATION_TIME))
                .map_err(|_| {
                    GoodFirstIssuesError::Cache("Invalid Cache-Control header value".to_string())
                })?;
            let x_cache = HeaderValue::from_str("HIT").map_err(|_| {
                GoodFirstIssuesError::Cache("Invalid X-Cache header value".to_string())
            })?;
            let age = Utc::now() - v.last_modified;
            let age = HeaderValue::from_str(&age.num_seconds().to_string())
                .map_err(|_| GoodFirstIssuesError::Cache("Invalid Age header value".to_string()))?;

            headers.insert("Cache-Control", max_age);
            headers.insert("X-Cache", x_cache);
            headers.insert("Age", age);
        }
        None => {
            let max_age = HeaderValue::from_str(&format!("max-age={}", CACHE_EXPIRATION_TIME))
                .map_err(|_| {
                    GoodFirstIssuesError::Cache("Invalid Cache-Control header value".to_string())
                })?;
            let x_cache = HeaderValue::from_str("MISS").map_err(|_| {
                GoodFirstIssuesError::Cache("Invalid X-Cache header value".to_string())
            })?;
            let age = HeaderValue::from_str(&0.to_string())
                .map_err(|_| GoodFirstIssuesError::Cache("Invalid Age header value".to_string()))?;

            headers.insert("Cache-Control", max_age);
            headers.insert("X-Cache", x_cache);
            headers.insert("Age", age);
        }
    };

    Ok(headers)
}
