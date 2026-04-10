use crate::{
    errors::{AppError, Result},
    models::{QueryParams, RawResponse},
};

pub fn validate_params(params: QueryParams) -> Result<String> {
    let name = params.name.trim();
    if name.is_empty() {
        return Err(AppError::BadRequest(
            "Missing or empty name parameter".to_string(),
        ));
    }

    if let Ok(value) = serde_json::from_str::<serde_json::Value>(name)
        && !value.is_string()
    {
        return Err(AppError::UnprocessableEntity(
            "name is not a string".to_string(),
        ));
    }

    Ok(name.to_string())
}

pub async fn fetch_gender_data(name: &str) -> Result<(String, u64, f64)> {
    let client = reqwest::Client::new();
    let api_response = client
        .get("https://api.genderize.io/")
        .query(&[("name", name)])
        .send()
        .await
        .map_err(|e| AppError::ServiceUnavailable(e.to_string()))?;

    let genderize: RawResponse = api_response
        .json()
        .await
        .map_err(|e| AppError::ServiceUnavailable(e.to_string()))?;

    let gender = match genderize.gender {
        Some(val) if !val.is_empty() => val,
        _ => {
            return Err(AppError::UnprocessableEntity(
                "No prediction available for the provided name".to_string(),
            ));
        }
    };

    let count = genderize.count.unwrap_or(0);
    if count == 0 {
        return Err(AppError::UnprocessableEntity(
            "No prediction available for the provided name".to_string(),
        ));
    }

    let probability = genderize.probability.unwrap_or(0.0);

    Ok((gender, count, probability))
}
