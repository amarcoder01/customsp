use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use log::info;

mod config;
mod handlers;
mod models;
mod services;

use config::AppConfig;
use services::database::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    info!("🚀 Starting SpeedTestPro Backend");
    
    // Load configuration
    let config = AppConfig::from_env();
    info!("📍 Server: {} ({})", config.server_name, config.server_id);
    info!("🌐 Binding to {}:{}", config.bind_host, config.bind_port);
    
    // Initialize database
    let database = Database::new(&config.database_path)
        .await
        .expect("Failed to initialize database");
    
    info!("💾 Database initialized");
    
    let db_data = web::Data::new(database);
    let config_data = web::Data::new(config.clone());
    
    // Start HTTP server
    info!("✅ Server ready at http://{}:{}", config.bind_host, config.bind_port);
    
    HttpServer::new(move || {
        let cors = Cors::permissive(); // Configure CORS properly in production
        
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(db_data.clone())
            .app_data(config_data.clone())
            .configure(handlers::configure_routes)
    })
    .bind((config.bind_host.as_str(), config.bind_port))?
    .workers(2) // Limited workers for 1 vCPU server
    .run()
    .await
}
