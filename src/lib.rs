pub mod errors;
pub mod handlers;
pub mod models;
pub mod utils;

use crate::handlers::process_gender;
use tower_http::cors::{Any, CorsLayer};

pub fn create_app() -> axum::Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    axum::Router::new()
        .route("/api/classify", axum::routing::get(process_gender))
        .layer(cors)
}
