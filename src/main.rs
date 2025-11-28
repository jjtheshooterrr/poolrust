mod chemistry;
mod image_pipeline;
mod types;

use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
    body::Bytes,
};
use std::net::SocketAddr;
use types::{AnalysisResult, PAD_LAYOUT};

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new()
        .route("/analyze-strip", post(analyze_strip))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)); // 10MB limit

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn analyze_strip(body: Bytes) -> impl IntoResponse {
    // 1. Load Image
    let img = match image_pipeline::load_image_from_bytes(&body) {
        Ok(img) => img,
        Err(e) => return (StatusCode::BAD_REQUEST, format!("Failed to load image: {}", e)).into_response(),
    };

    // 2. Find Strip and Warp
    let warped_img = match image_pipeline::find_strip_and_warp(&img) {
        Ok(img) => img,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to process image: {}", e)).into_response(),
    };

    // 3. Extract Pad Colors
    let pad_colors = image_pipeline::get_pad_colors(&warped_img, &PAD_LAYOUT);

    // 4. Map Colors to Values
    let chemical_defs = chemistry::get_chemical_definitions();
    let mut free_chlorine = 0.0;
    let mut ph = 0.0;
    let mut alkalinity = 0.0;
    let mut cyanuric_acid = 0.0;
    let mut notes = Vec::new();

    for (name, (r, g, b)) in pad_colors {
        if let Some(value) = chemistry::map_color_to_value(r, g, b, &name, &chemical_defs) {
            match name.as_str() {
                "free_chlorine" => free_chlorine = value,
                "ph" => ph = value,
                "total_alkalinity" => alkalinity = value,
                "cyanuric_acid" => cyanuric_acid = value,
                _ => {}
            }
        } else {
            notes.push(format!("Could not determine value for {}", name));
        }
    }

    // Simple logic for notes (demo purposes)
    if ph > 7.6 {
        notes.push("pH is high. Consider adding acid.".to_string());
    } else if ph < 7.2 {
        notes.push("pH is low. Consider adding base.".to_string());
    }

    let result = AnalysisResult {
        free_chlorine_ppm: free_chlorine,
        ph,
        total_alkalinity_ppm: alkalinity,
        cyanuric_acid_ppm: cyanuric_acid,
        notes,
    };

    (StatusCode::OK, axum::Json(result)).into_response()
}
