use axum::{http::StatusCode, response::{IntoResponse, Response}};




pub enum ApiError {
    Ok {message: String},
    Created {message: String},
    BadRequest {message: String},
    Unauthorized {message: String},
    Forbidden {message: String},
    NotFound  {message: String},
    Conflict  {message: String},
    UnprocessableEntity  {message: String},
    TooManyRequests  {message: String},
    InternalServerError  {message: String},
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Ok { message } => (StatusCode::OK, message).into_response(),
            ApiError::Created { message } => (StatusCode::CREATED, message).into_response(),
            ApiError::BadRequest { message } => (StatusCode::OK, message).into_response(),
            ApiError::Unauthorized { message } => (StatusCode::OK, message).into_response(),
            ApiError::Forbidden { message } => (StatusCode::OK, message).into_response(),
            ApiError::NotFound { message } => (StatusCode::OK, message).into_response(),
            ApiError::Conflict { message } => (StatusCode::OK, message).into_response(),
            ApiError::UnprocessableEntity { message } => (StatusCode::OK, message).into_response(),
            ApiError::TooManyRequests { message } => (StatusCode::OK, message).into_response(),
            ApiError::InternalServerError { message } => (StatusCode::OK, message).into_response(),
        }
    }
}