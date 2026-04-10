use axum::{
    extract::{Query, rejection::QueryRejection},
    response::IntoResponse,
};
use chrono::Utc;

use crate::{
    errors::AppError,
    models::{ApiResponse, JsonResponse, QueryParams},
    utils::{fetch_gender_data, validate_params},
};

pub async fn process_gender(
    params: Result<Query<QueryParams>, QueryRejection>,
) -> impl IntoResponse {
    let params = match params {
        Ok(Query(param)) => param,
        Err(e) => {
            tracing::error!("Invalid query parameters: {}", e);
            return AppError::BadRequest("Missing or empty name parameter".to_string())
                .into_response();
        }
    };

    let name = match validate_params(params) {
        Ok(n) => n,
        Err(e) => return e.into_response(),
    };

    let (gender, count, probability) = match fetch_gender_data(&name).await {
        Ok(data) => data,
        Err(e) => return e.into_response(),
    };

    let is_confident = probability >= 0.7 && count >= 100;

    let response = ApiResponse {
        name,
        gender,
        probability,
        sample_size: count,
        is_confident,
        processed_at: Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
    };

    JsonResponse::success(response).into_response()
}
