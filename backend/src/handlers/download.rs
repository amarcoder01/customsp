/// HTTP Download Endpoint for Speed Testing
/// 
/// Provides large file downloads for testing download speeds.
/// This is an alternative to WebSocket-based testing.

use actix_web::{web, HttpResponse, Result};
use bytes::Bytes;
use log::info;
use rand::Rng;

/// Generate and serve random data for download testing
pub async fn download_test(
    query: web::Query<DownloadQuery>,
) -> Result<HttpResponse> {
    let size_mb = query.size_mb.unwrap_or(10).min(100); // Max 100MB
    let size_bytes = size_mb * 1024 * 1024;
    
    info!("📥 HTTP download test: {} MB", size_mb);
    
    // Generate random data
    let mut rng = rand::thread_rng();
    let data: Vec<u8> = (0..size_bytes)
        .map(|_| rng.gen::<u8>())
        .collect();
    
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .insert_header(("Content-Disposition", format!("attachment; filename=\"speedtest-{}.bin\"", size_mb)))
        .insert_header(("Cache-Control", "no-cache, no-store, must-revalidate"))
        .body(Bytes::from(data)))
}

/// Accept upload data for testing upload speeds
pub async fn upload_test(
    body: web::Bytes,
) -> Result<HttpResponse> {
    let size_mb = body.len() as f64 / 1024.0 / 1024.0;
    
    info!("📤 HTTP upload test: {:.2} MB received", size_mb);
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "bytes_received": body.len(),
        "size_mb": size_mb,
    })))
}

#[derive(serde::Deserialize)]
pub struct DownloadQuery {
    pub size_mb: Option<usize>,
}
