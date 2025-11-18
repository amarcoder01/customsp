use actix_web::web;

pub mod health;
pub mod servers;
pub mod test;
pub mod enhanced_test;
pub mod download;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health::health_check))
            .route("/servers", web::get().to(servers::get_servers))
            // Basic test endpoints
            .route("/test/start", web::post().to(test::start_test))
            .route("/test/{id}", web::get().to(test::get_result))
            .route("/test/history", web::get().to(test::get_history))
            // Enhanced test endpoints with all features
            .route("/test/enhanced/start", web::post().to(enhanced_test::start_enhanced_test))
            .route("/ws/enhanced/{test_id}", web::get().to(enhanced_test::websocket_enhanced_test))
            .route("/api/enhanced/result/{test_id}", web::get().to(enhanced_test::get_enhanced_result))
            // HTTP-based speed test endpoints
            .route("/api/download", web::get().to(download::download_test))
            .route("/api/upload", web::post().to(download::upload_test)),
    )
    .route("/ws/test/{id}", web::get().to(test::websocket_test))
    .route("/ws/enhanced/{id}", web::get().to(enhanced_test::websocket_enhanced_test));
}
