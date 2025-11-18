use actix_web::{web, HttpRequest, HttpResponse, Result, Error};
use actix_ws::Message;
use log::{info, error};
use uuid::Uuid;

use crate::config::AppConfig;
use crate::models::{StartTestRequest, StartTestResponse, TestResult};
use crate::services::database::Database;
use crate::services::measurement::MeasurementEngine;
use crate::services::real_measurement::RealMeasurementEngine;

pub async fn start_test(
    req: web::Json<StartTestRequest>,
    config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    info!("Starting new speed test");
    
    let test_id = Uuid::new_v4().to_string();
    let duration_ms = req
        .duration_ms
        .unwrap_or(config.default_test_duration_ms)
        .clamp(config.min_test_duration_ms, config.max_test_duration_ms);
    
    let response = StartTestResponse {
        test_id: test_id.clone(),
        server_id: config.server_id.clone(),
        websocket_url: format!("ws://{}:{}/ws/test/{}", config.server_ip, config.bind_port, test_id),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn websocket_test(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<String>,
    config: web::Data<AppConfig>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let test_id = path.into_inner();
    info!("WebSocket connection established for test: {}", test_id);
    
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;
    
    let config = config.get_ref().clone();
    let db = db.get_ref().clone();
    
    // Get client IP
    let client_ip = req
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    
    // Spawn a task to handle the test
    actix_web::rt::spawn(async move {
        let mut stream = stream;
        
        // Use REAL measurement engine with actual data transfer
        let engine = RealMeasurementEngine::new(config.clone());
        
        // Run the speed test with REAL bytes
        match engine.run_full_test(&test_id, &mut session, client_ip).await {
            Ok(result) => {
                info!("Test completed successfully: {}", test_id);
                
                // Save result to database
                if let Err(e) = db.save_test_result(&result).await {
                    error!("Failed to save test result: {}", e);
                }
                
                // Send final result
                let result_json = serde_json::to_string(&result).unwrap();
                let _ = session.text(result_json).await;
            }
            Err(e) => {
                error!("Test failed: {}", e);
                let error_msg = format!(r#"{{"error": "{}"}}"#, e);
                let _ = session.text(error_msg).await;
            }
        }
        
        let _ = session.close(None).await;
    });
    
    Ok(res)
}

pub async fn get_result(
    path: web::Path<String>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let test_id = path.into_inner();
    info!("Fetching result for test: {}", test_id);
    
    match db.get_test_result(&test_id).await {
        Ok(Some(result)) => Ok(HttpResponse::Ok().json(result)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Test result not found"
        }))),
        Err(e) => {
            error!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch test result"
            })))
        }
    }
}

pub async fn get_history(
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    info!("Fetching test history");
    
    match db.get_test_history(20).await {
        Ok(results) => Ok(HttpResponse::Ok().json(results)),
        Err(e) => {
            error!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch test history"
            })))
        }
    }
}
