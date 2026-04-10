use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct RawResponse {
    pub count: Option<u64>,
    pub gender: Option<String>,
    pub name: String,
    pub probability: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub name: String,
    pub gender: String,
    pub probability: f64,
    pub sample_size: u64,
    pub is_confident: bool,
    pub processed_at: String,
}

#[derive(Debug, Serialize)]
pub struct JsonResponse {
    pub status: String,
    pub data: ApiResponse,
}

impl JsonResponse {
    pub fn success(data: ApiResponse) -> Self {
        Self {
            status: "success".to_string(),
            data,
        }
    }
}

impl IntoResponse for JsonResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::OK;
        let body =
            serde_json::to_string(&self).map_err(|e| AppError::InternalServerError(e.to_string()));
        (status, [("content-type", "application/json")], body).into_response()
    }
}
