use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum ApiError {
    Ok { message: String },
    Created { message: String },
    BadRequest { message: String },
    Unauthorized { message: String },
    Forbidden { message: String },
    NotFound { message: String },
    NotAllowed { message: Option<String> },
    Conflict { message: String },
    UnprocessableEntity { message: String },
    TooManyRequests { message: String },
    InternalServerError { message: String },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Ok { message } => (StatusCode::OK, message).into_response(),
            ApiError::Created { message } => (StatusCode::CREATED, message).into_response(),
            ApiError::BadRequest { message } => (StatusCode::BAD_REQUEST, message).into_response(),
            ApiError::Unauthorized { message } => {
                (StatusCode::UNAUTHORIZED, message).into_response()
            }
            ApiError::Forbidden { message } => (StatusCode::FORBIDDEN, message).into_response(),
            ApiError::NotFound { message } => (StatusCode::NOT_FOUND, message).into_response(),
            ApiError::Conflict { message } => (StatusCode::CONFLICT, message).into_response(),
            ApiError::UnprocessableEntity { message } => {
                (StatusCode::UNPROCESSABLE_ENTITY, message).into_response()
            }
            ApiError::NotAllowed { message } => (
                StatusCode::METHOD_NOT_ALLOWED,
                message.ok_or(String::from("Method not allowed")),
            )
                .into_response(),
            ApiError::TooManyRequests { message } => {
                (StatusCode::TOO_MANY_REQUESTS, message).into_response()
            }
            ApiError::InternalServerError { message } => {
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
        }
    }
}
