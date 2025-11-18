/// Test program for AIM (Aggregated Internet Measurement) Scoring
/// 
/// This demonstrates how we translate raw metrics into user-friendly
/// quality scores for different use cases.
///
/// Run with: cargo run --example test_aim_scoring

use speedtest_pro_backend::models::TestResult;
use speedtest_pro_backend::services::loaded_latency::{LoadedLatencyResult, BufferbloatGrade};
use speedtest_pro_backend::services::aim_scoring::AIMCalculator;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    println!("\n🎯 SpeedTestPro - AIM Scoring System Demo");
    println!("=========================================\n");
    println!("AIM translates raw metrics into meaningful quality scores:");
    println!("• Gaming: Low latency and stable connection");
    println!("• Streaming: High download speed and consistency");
    println!("• Video Conferencing: Good upload and low latency");
    println!("• General Browsing: Overall balance\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // Test Case 1: Excellent Connection
    println!("📊 TEST CASE 1: Excellent Fiber Connection");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    test_connection("Excellent Fiber", 450.0, 50.0, 12.0, 3.0, 15.0, 18.0, 22.0);
    
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // Test Case 2: Good Cable Connection
    println!("📊 TEST CASE 2: Good Cable Internet");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    test_connection("Good Cable", 200.0, 20.0, 25.0, 8.0, 28.0, 45.0, 65.0);
    
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // Test Case 3: Fair Connection with Bufferbloat
    println!("📊 TEST CASE 3: Connection with Bufferbloat");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    test_connection("Bufferbloat Issue", 300.0, 30.0, 15.0, 12.0, 18.0, 95.0, 180.0);
    
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // Test Case 4: Poor Connection
    println!("📊 TEST CASE 4: Poor DSL Connection");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    test_connection("Poor DSL", 8.0, 1.0, 80.0, 25.0, 85.0, 120.0, 250.0);
    
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // Test Case 5: Asymmetric Upload Issue
    println!("📊 TEST CASE 5: Good Download, Poor Upload");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    test_connection("Asymmetric", 400.0, 5.0, 18.0, 6.0, 20.0, 25.0, 150.0);
    
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ AIM Scoring Demo Complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}

fn test_connection(
    name: &str,
    download_mbps: f64,
    upload_mbps: f64,
    idle_latency_ms: f64,
    jitter_ms: f64,
    idle_rpm_base: f64,
    download_latency_ms: f64,
    upload_latency_ms: f64,
) {
    println!("Connection: {}", name);
    println!("Raw Metrics:");
    println!("  Download: {:.1} Mbps", download_mbps);
    println!("  Upload:   {:.1} Mbps", upload_mbps);
    println!("  Idle Latency: {:.1}ms", idle_latency_ms);
    println!("  Download Loaded: {:.1}ms", download_latency_ms);
    println!("  Upload Loaded: {:.1}ms", upload_latency_ms);
    println!("  Jitter: {:.1}ms\n", jitter_ms);
    
    // Create mock test result
    let test_result = TestResult {
        id: "test-123".to_string(),
        server_id: "mumbai-01".to_string(),
        timestamp: chrono::Utc::now(),
        download_mbps,
        upload_mbps,
        latency_ms: idle_latency_ms,
        jitter_ms,
        protocol: "TCP".to_string(),
        client_ip: "1.2.3.4".to_string(),
        test_duration_ms: 10000,
    };
    
    // Create mock loaded latency result
    let loaded_latency = LoadedLatencyResult {
        idle_min_ms: idle_latency_ms - 2.0,
        idle_max_ms: idle_latency_ms + 5.0,
        idle_avg_ms: idle_latency_ms,
        idle_median_ms: idle_latency_ms,
        idle_samples: vec![idle_latency_ms],
        
        download_min_ms: download_latency_ms - 5.0,
        download_max_ms: download_latency_ms + 10.0,
        download_avg_ms: download_latency_ms,
        download_median_ms: download_latency_ms,
        download_samples: vec![download_latency_ms],
        
        upload_min_ms: upload_latency_ms - 8.0,
        upload_max_ms: upload_latency_ms + 15.0,
        upload_avg_ms: upload_latency_ms,
        upload_median_ms: upload_latency_ms,
        upload_samples: vec![upload_latency_ms],
        
        bufferbloat_download_ms: download_latency_ms - idle_latency_ms,
        bufferbloat_upload_ms: upload_latency_ms - idle_latency_ms,
        bufferbloat_download_ratio: (download_latency_ms - idle_latency_ms) / idle_latency_ms,
        bufferbloat_upload_ratio: (upload_latency_ms - idle_latency_ms) / idle_latency_ms,
        bufferbloat_grade: BufferbloatGrade::A,
        
        idle_rpm: 60000.0 / idle_latency_ms,
        download_rpm: 60000.0 / download_latency_ms,
        upload_rpm: 60000.0 / upload_latency_ms,
    };
    
    // Calculate AIM scores
    let aim_scores = AIMCalculator::calculate_all_scores(&test_result, &loaded_latency);
    
    // Display summary
    println!("{}\n", aim_scores.summary());
    
    // Show detailed reports for each use case
    println!("📋 Detailed Assessments:\n");
    
    println!("{}", aim_scores.detailed_report("gaming"));
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    println!("{}", aim_scores.detailed_report("streaming"));
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    println!("{}", aim_scores.detailed_report("conferencing"));
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    println!("{}", aim_scores.detailed_report("browsing"));
    
    // JSON output
    println!("\n📄 JSON Output:");
    match serde_json::to_string_pretty(&aim_scores) {
        Ok(json) => println!("{}\n", json),
        Err(e) => println!("Error: {}\n", e),
    }
}
