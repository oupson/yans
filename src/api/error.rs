use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    Network(#[from] reqwest::Error),
    #[error("invalid push url")]
    InvalidPushUrl(),
    #[error("device not found")]
    DeviceNotFound(),
}

#[derive(Serialize)]
pub(crate) struct ErrorResponse {
    error_type: String,
    error_message: Option<String>,
}

impl ErrorResponse {
    pub(crate) fn new<S1, S2>(error_type: S1, error_message: Option<S2>) -> Self
    where
        S1: ToString,
        S2: ToString,
    {
        Self {
            error_type: error_type.to_string(),
            error_message: error_message.map(|s| s.to_string()),
        }
    }
}

#[cfg(debug_assertions)]
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::InvalidPushUrl() => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new("Bad Request", Some("Invalid Push URL"))),
            )
                .into_response(),
            Self::DeviceNotFound() => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse::new("Unauthorized", Some("Device not found"))),
            )
                .into_response(),
            e => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "Internal Server Error",
                    Some(format!("{}", e)),
                )),
            )
                .into_response(),
        }
    }
}

#[cfg(not(debug_assertions))]
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::InvalidPushUrl() => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new("Bad Request", Some("Invalid Push URL"))),
            )
                .into_response(),
            Self::DeviceNotFound() => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse::new("Unauthorized", None)),
            )
                .into_response(),
            e => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("Internal Server Error", None::<String>)),
            )
                .into_response(),
        }
    }
}
