use std::time::Duration;

use crate::{errors::ApiError, handlers::survey::survey_routes};
use axum::{
    Router,
    http::{StatusCode, Uri},
};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer,
};

pub mod survey;

pub fn all_routes() -> Router {
    Router::new()
        .merge(survey_routes())
        .method_not_allowed_fallback(method_fallback_route)
        .fallback(fallback_route)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::new())
                .layer(TimeoutLayer::with_status_code(
                    StatusCode::REQUEST_TIMEOUT,
                    Duration::from_secs(30),
                ))
                .layer(CompressionLayer::new()),
        )
}

pub async fn fallback_route(uri: Uri) -> ApiError {
    ApiError::NotFound {
        message: format!("Not found: {uri}"),
    }
}

pub async fn method_fallback_route(uri: Uri) -> ApiError {
    ApiError::NotAllowed {
        message: Some(format!("Method not allowed for Uri: {uri}")),
    }
}
