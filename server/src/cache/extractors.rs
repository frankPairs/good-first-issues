//! ExtratCache key is an Axum extractor that creates a cache key from the request path and query parameters.
//!
//! The key is constructed by concatenating the path and query parameters. Query parameters are sorted alphabetically in order to ensure the same query parameters result in the same key,
//! independently of the order they were provided in the request.
//!
//! It separates the path segments and query parameters by a colon (:).
//!
//! For example, given the following request:
//!
//! GET <https://domain.com/api/v1/users?name=John&age=30>
//!
//! The key generated would be:
//!
//! api:v1:users:age=30:name=John
//!
//! When the request does contain nor path neither query parameters, it returns a 400 Bad Request error as the key would be empty.
//!
use axum::{
    async_trait,
    extract::{FromRequestParts, OriginalUri},
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};

use itertools::{sorted, Itertools};

const CACHE_KEY_DELIMITER: &str = ":";

pub struct ExtractCacheKey(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractCacheKey
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let original_uri = parts.extract::<OriginalUri>().await.unwrap();

        let formatted_path = original_uri
            .path()
            .to_string()
            .replace("/", CACHE_KEY_DELIMITER);
        let query_params = original_uri.query().unwrap_or("");
        let sorted_params = sorted(query_params.split("&")).join(CACHE_KEY_DELIMITER);

        let mut cache_key = [formatted_path, sorted_params].join(CACHE_KEY_DELIMITER);

        if cache_key.starts_with(CACHE_KEY_DELIMITER) {
            cache_key.remove(0);
        }

        if cache_key.ends_with(CACHE_KEY_DELIMITER) {
            cache_key.pop();
        }

        if cache_key.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "Invalid key"));
        }

        Ok(ExtractCacheKey(cache_key))
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
        routing::get,
        Router,
    };
    use http_body_util::BodyExt;
    use tower::util::ServiceExt;

    use super::*;

    async fn handler_with_extract_cache_key(key: ExtractCacheKey) -> String {
        key.0
    }

    fn app() -> Router {
        Router::new()
            .route("/api/v1/test", get(handler_with_extract_cache_key))
            .route("/", get(handler_with_extract_cache_key))
    }

    #[tokio::test]
    async fn test_with_path() {
        let app = app();

        let res = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/api/v1/test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let status = res.status();
        let res_body = res.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(res_body.to_vec()).unwrap();

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "api:v1:test");
    }

    #[tokio::test]
    async fn test_with_path_and_query() {
        let app = app();

        let res = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/api/v1/test?name=John&age=30")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let status = res.status();
        let res_body = res.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(res_body.to_vec()).unwrap();

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "api:v1:test:age=30:name=John");
    }

    #[tokio::test]
    async fn test_empty_params_returns_bad_request() {
        let app = app();

        let res = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let status = res.status();

        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_nested_routes() {
        fn app_with_nested_routes() -> Router {
            Router::new().nest(
                "/api/v1",
                Router::new().route("/test", get(handler_with_extract_cache_key)),
            )
        }

        let app = app_with_nested_routes();

        let res = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/api/v1/test?name=John&age=30")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let status = res.status();
        let res_body = res.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(res_body.to_vec()).unwrap();

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "api:v1:test:age=30:name=John");
    }
}
