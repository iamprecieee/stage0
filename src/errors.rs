use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct JsonError {
    pub message: String,
    pub status: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    UnprocessableEntity(String),
    #[error("{0}")]
    ServiceUnavailable(String),
    #[error("{0}")]
    InternalServerError(String),
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    TryInitError(#[from] tracing_subscriber::util::TryInitError),
}

impl AppError {
    pub fn status_code(&self) -> u16 {
        match self {
            AppError::BadRequest(_) => 400,
            AppError::UnprocessableEntity(_) => 422,
            AppError::ServiceUnavailable(_) => 502,
            AppError::InternalServerError(_) | AppError::IoError(_) | AppError::TryInitError(_) => {
                500
            }
        }
    }

    pub fn to_json_error(&self) -> JsonError {
        JsonError {
            message: match self {
                AppError::BadRequest(msg) | AppError::UnprocessableEntity(msg) => msg.to_string(),
                AppError::ServiceUnavailable(msg) | AppError::InternalServerError(msg) => {
                    tracing::error!("{}", msg);
                    "Upstream or server failure".to_string()
                }
                AppError::IoError(msg) => {
                    tracing::error!("{}", msg);
                    "Upstream or server failure".to_string()
                }
                AppError::TryInitError(msg) => {
                    tracing::error!("{}", msg);
                    "Upstream or server failure".to_string()
                }
            },
            status: "error".to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status_code()).unwrap();
        let body = serde_json::to_string(&self.to_json_error()).unwrap_or_else(|_| {
            r#"{"status": "error", "message": "Upstream or server failure"}"#.to_string()
        });
        (status, [("content-type", "application/json")], body).into_response()
    }
}
