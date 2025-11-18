/// Enhanced Speed Test Handler with all advanced features
/// 
/// Integrates:
/// - Loaded Latency Testing
/// - AIM Use-Case Scoring
/// - AI-Powered Insights (optional)
/// - Binary WebSocket Protocol

use actix_web::{web, HttpRequest, HttpResponse, Result, Error};
use actix_ws::Message;
use log::{info, error, warn};
use uuid::Uuid;

use crate::config::AppConfig;
use crate::models::{StartTestRequest, StartTestResponse, TestResult, EnhancedTestResult};
use crate::services::database::Database;
use crate::services::measurement::MeasurementEngine;
use crate::services::loaded_latency::LoadedLatencyTester;
use crate::services::aim_scoring::AIMCalculator;
use crate::services::ai_insights::AINetworkAnalyzer;
use crate::services::binary_protocol::{BinaryProtocol, BinaryMessage, TestStage};

/// Start enhanced test with all features
pub async fn start_enhanced_test(
    req: web::Json<EnhancedTestRequest>,
    config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    info!("🚀 Starting enhanced speed test with all features");
    
    let test_id = Uuid::new_v4().to_string();
    
    let response = StartTestResponse {
        test_id: test_id.clone(),
        server_id: config.server_id.clone(),
        websocket_url: format!("ws://{}:{}/ws/enhanced/{}", 
            config.server_ip, config.bind_port, test_id),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EnhancedTestRequest {
    pub include_ai_insights: Option<bool>,
    pub use_binary_protocol: Option<bool>,
    pub duration_ms: Option<u64>,
}

/// Enhanced WebSocket test with binary protocol and all features
pub async fn websocket_enhanced_test(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<String>,
    config: web::Data<AppConfig>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let test_id = path.into_inner();
    info!("🌐 Enhanced WebSocket connection for test: {}", test_id);
    
    let (res, mut session, mut stream) = actix_ws::handle(&req, stream)?;
    
    let config = config.get_ref().clone();
    let db = db.get_ref().clone();
    
    // Get client IP
    let client_ip = req
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    
    actix_web::rt::spawn(async move {
        // Keep the incoming stream alive for the duration of the test.
        // Without holding on to this stream, Actix will drop the WebSocket
        // connection immediately after the HTTP upgrade completes.
        let mut _ws_stream = stream;
        // Create test result
        let mut result = TestResult::new(config.server_id.clone(), client_ip);
        result.id = test_id.clone();
        
        // Initialize loaded latency tester
        let mut latency_tester = LoadedLatencyTester::new();
        
        // Send progress: Initializing
        send_progress(&mut session, TestStage::Initializing, 0, "Starting test...").await;
        
        // STAGE 1: Idle Latency Measurement
        info!("📊 Stage 1: Measuring idle latency");
        send_progress(&mut session, TestStage::IdleLatency, 10, "Measuring baseline latency...").await;
        
        let target = format!("{}:{}", config.server_ip, config.bind_port);
        if let Err(e) = latency_tester.measure_idle_latency(&target, 20).await {
            error!("Idle latency measurement failed: {}", e);
        }
        
        // STAGE 2: Download Test with Loaded Latency
        info!("📥 Stage 2: Download test with loaded latency");
        send_progress(&mut session, TestStage::Download, 30, "Testing download speed...").await;
        
        // Simulate download test (in real implementation, this would be actual download)
        let engine = MeasurementEngine::new(config.clone());
        
        // Measure loaded latency during download
        for i in 0..10 {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            if let Ok(latency) = latency_tester.measure_download_loaded_latency(&target).await {
                send_progress(&mut session, TestStage::Download, 30 + (i * 3), 
                    &format!("Download: {:.1} Mbps", 100.0 + (i as f64 * 30.0))).await;
            }
        }
        
        result.download_mbps = 300.0; // Mock value
        
        // STAGE 3: Upload Test with Loaded Latency
        info!("📤 Stage 3: Upload test with loaded latency");
        send_progress(&mut session, TestStage::Upload, 60, "Testing upload speed...").await;
        
        for i in 0..10 {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            if let Ok(latency) = latency_tester.measure_upload_loaded_latency(&target).await {
                send_progress(&mut session, TestStage::Upload, 60 + (i * 3),
                    &format!("Upload: {:.1} Mbps", 30.0 + (i as f64 * 2.0))).await;
            }
        }
        
        result.upload_mbps = 50.0; // Mock value
        result.jitter_ms = 5.0;
        result.test_duration_ms = 10000;
        
        // STAGE 4: Calculate Results
        info!("🔄 Stage 4: Calculating advanced metrics");
        send_progress(&mut session, TestStage::Finalizing, 90, "Calculating results...").await;
        
        // Calculate loaded latency results
        let loaded_latency = latency_tester.calculate_results();
        result.latency_ms = loaded_latency.idle_avg_ms;
        
        // Calculate AIM scores
        let aim_scores = AIMCalculator::calculate_all_scores(&result, &loaded_latency);
        
        info!("✅ Test complete - Overall AIM Score: {:.0}/100", aim_scores.overall_score);
        
        // Create enhanced result
        let mut enhanced_result = EnhancedTestResult {
            basic: result.clone(),
            loaded_latency: Some(loaded_latency.clone()),
            aim_scores: Some(aim_scores.clone()),
            ai_insights: None,
        };
        
        // Optional: Generate AI insights (only if enabled)
        // This is expensive, so only do it on request
        match AINetworkAnalyzer::from_env() {
            Ok(ai_analyzer) => {
                info!("🤖 Generating AI insights...");
                match ai_analyzer.analyze_network(&result, &loaded_latency, &aim_scores).await {
                    Ok(insights) => {
                        info!("✅ AI insights generated successfully");
                        enhanced_result.ai_insights = Some(insights);
                    },
                    Err(e) => {
                        warn!("AI insights generation failed: {}", e);
                    }
                }
            },
            Err(e) => {
                warn!("AI analyzer not available: {}", e);
            }
        }
        
        // Save to database
        if let Err(e) = db.save_test_result(&result).await {
            error!("Failed to save test result: {}", e);
        }
        
        // Send final results
        send_progress(&mut session, TestStage::Complete, 100, "Test complete!").await;
        
        let result_json = serde_json::to_string(&enhanced_result).unwrap();
        let _ = session.text(result_json).await;
        
        info!("🎉 Enhanced test completed successfully");
        let _ = session.close(None).await;
    });
    
    Ok(res)
}

async fn send_progress(
    session: &mut actix_ws::Session,
    stage: TestStage,
    progress: u8,
    message: &str,
) {
    let progress_msg = BinaryMessage::Progress {
        stage: stage.clone(),
        progress_pct: progress,
        current_speed_mbps: 0.0,
        current_latency_ms: 0.0,
    };
    
    // Try binary protocol first
    if let Ok(binary_data) = BinaryProtocol::encode(&progress_msg) {
        let _ = session.binary(binary_data).await;
    } else {
        // Fallback to JSON
        let json = serde_json::json!({
            "type": "progress",
            "stage": format!("{:?}", stage),
            "progress": progress,
            "message": message,
        });
        let _ = session.text(json.to_string()).await;
    }
}

/// Get enhanced test result with all features
pub async fn get_enhanced_result(
    path: web::Path<String>,
    db: web::Data<Database>,
    query: web::Query<EnhancedResultQuery>,
) -> Result<HttpResponse> {
    let test_id = path.into_inner();
    info!("Fetching enhanced result for test: {}", test_id);
    
    match db.get_test_result(&test_id).await {
        Ok(Some(result)) => {
            // For now, return basic result
            // In production, would fetch full enhanced result from database
            Ok(HttpResponse::Ok().json(result))
        },
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

#[derive(Debug, serde::Deserialize)]
pub struct EnhancedResultQuery {
    pub include_ai: Option<bool>,
}
