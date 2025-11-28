mod chemistry;
mod image_pipeline;
mod types;

use axum::{
    extract::{Query, DefaultBodyLimit},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
    body::Bytes,
};
use serde::Deserialize;
use std::net::SocketAddr;
use types::{AnalysisResult, Brand};

#[derive(Deserialize)]
struct AnalyzeQuery {
    brand: Option<String>,
}

fn parse_brand(s: &str) -> Brand {
    match s.to_lowercase().as_str() {
        "clorox" => Brand::Clorox6Way,
        "aquachek" | "aquachek7" => Brand::AquaChek7Way,
        _ => Brand::Hth6Way, // default / fallback
    }
}

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new()
        .route("/analyze-strip", post(analyze_strip))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)); // 10MB limit

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn analyze_strip(
    Query(q): Query<AnalyzeQuery>,
    body: Bytes,
) -> impl IntoResponse {
    let brand = q.brand.as_deref().map(parse_brand).unwrap_or(Brand::Hth6Way);

    // Process image
    match image_pipeline::process_image_for_brand(&body, brand) {
        Ok(result) => (StatusCode::OK, axum::Json(result)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Image processing error: {}", e)).into_response(),
    }
}
