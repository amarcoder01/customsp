use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub id: String,
    pub name: String,
    pub location: Location,
    pub available: bool,
    pub load: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub duration_ms: u64,
    pub protocol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub id: String,
    pub server_id: String,
    pub timestamp: DateTime<Utc>,
    pub download_mbps: f64,
    pub upload_mbps: f64,
    pub latency_ms: f64,
    pub jitter_ms: f64,
    pub protocol: String,
    pub client_ip: String,
    pub test_duration_ms: u64,
}

/// Enhanced test result with all advanced features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedTestResult {
    // Basic test result
    #[serde(flatten)]
    pub basic: TestResult,
    
    // Loaded latency results
    pub loaded_latency: Option<crate::services::loaded_latency::LoadedLatencyResult>,
    
    // AIM scores
    pub aim_scores: Option<crate::services::aim_scoring::AIMScores>,
    
    // AI insights (optional - only if user requests)
    pub ai_insights: Option<crate::services::ai_insights::AIInsights>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestProgress {
    pub stage: String, // "latency", "download", "upload", "complete"
    pub progress: f32,  // 0.0 to 1.0
    pub current_speed_mbps: Option<f64>,
    pub current_latency_ms: Option<f64>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub active_tests: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartTestRequest {
    pub duration_ms: Option<u64>,
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartTestResponse {
    pub test_id: String,
    pub server_id: String,
    pub websocket_url: String,
}

impl TestResult {
    pub fn new(server_id: String, client_ip: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            server_id,
            timestamp: Utc::now(),
            download_mbps: 0.0,
            upload_mbps: 0.0,
            latency_ms: 0.0,
            jitter_ms: 0.0,
            protocol: "TCP".to_string(),
            client_ip,
            test_duration_ms: 0,
        }
    }
}

impl TestProgress {
    pub fn new(stage: &str, progress: f32, message: &str) -> Self {
        Self {
            stage: stage.to_string(),
            progress,
            current_speed_mbps: None,
            current_latency_ms: None,
            message: message.to_string(),
        }
    }
    
    pub fn with_speed(mut self, speed: f64) -> Self {
        self.current_speed_mbps = Some(speed);
        self
    }
    
    pub fn with_latency(mut self, latency: f64) -> Self {
        self.current_latency_ms = Some(latency);
        self
    }
}
