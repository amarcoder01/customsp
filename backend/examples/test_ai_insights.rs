/// Test program for AI-Powered Network Insights
/// 
/// This demonstrates how GPT-4 analyzes network test results
/// and provides intelligent recommendations
///
/// Run with: cargo run --example test_ai_insights

use speedtest_pro_backend::models::TestResult;
use speedtest_pro_backend::services::loaded_latency::{LoadedLatencyResult, BufferbloatGrade};
use speedtest_pro_backend::services::aim_scoring::AIMCalculator;
use speedtest_pro_backend::services::ai_insights::AINetworkAnalyzer;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    println!("\n🤖 SpeedTestPro - AI-Powered Network Insights");
    println!("==============================================\n");
    println!("Using GPT-4o-mini to analyze network performance");
    println!("and provide intelligent recommendations!\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // Initialize AI analyzer
    let ai_analyzer = match AINetworkAnalyzer::from_env() {
        Ok(analyzer) => analyzer,
        Err(e) => {
            println!("❌ Error initializing AI analyzer: {}", e);
            println!("\n💡 Make sure OPENAI_API_KEY is set in .env file\n");
            return;
        }
    };
    
    println!("✅ AI Analyzer initialized\n");
    
    // Test Case: Connection with Bufferbloat
    println!("📊 TEST CASE: Connection with Moderate Bufferbloat");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // Create mock test results
    let test_result = TestResult {
        id: "test-ai-001".to_string(),
        server_id: "mumbai-01".to_string(),
        timestamp: chrono::Utc::now(),
        download_mbps: 300.0,
        upload_mbps: 30.0,
        latency_ms: 15.0,
        jitter_ms: 12.0,
        protocol: "TCP".to_string(),
        client_ip: "1.2.3.4".to_string(),
        test_duration_ms: 10000,
    };
    
    let loaded_latency = LoadedLatencyResult {
        idle_min_ms: 13.0,
        idle_max_ms: 18.0,
        idle_avg_ms: 15.0,
        idle_median_ms: 15.0,
        idle_samples: vec![15.0],
        
        download_min_ms: 80.0,
        download_max_ms: 105.0,
        download_avg_ms: 95.0,
        download_median_ms: 93.0,
        download_samples: vec![95.0],
        
        upload_min_ms: 160.0,
        upload_max_ms: 200.0,
        upload_avg_ms: 180.0,
        upload_median_ms: 175.0,
        upload_samples: vec![180.0],
        
        bufferbloat_download_ms: 80.0,
        bufferbloat_upload_ms: 165.0,
        bufferbloat_download_ratio: 5.33,
        bufferbloat_upload_ratio: 11.0,
        bufferbloat_grade: BufferbloatGrade::C,
        
        idle_rpm: 4000.0,
        download_rpm: 632.0,
        upload_rpm: 333.0,
    };
    
    // Calculate AIM scores
    let aim_scores = AIMCalculator::calculate_all_scores(&test_result, &loaded_latency);
    
    println!("Raw Metrics:");
    println!("  Download: {:.1} Mbps", test_result.download_mbps);
    println!("  Upload: {:.1} Mbps", test_result.upload_mbps);
    println!("  Idle Latency: {:.1}ms", loaded_latency.idle_avg_ms);
    println!("  Download Loaded: {:.1}ms (+{:.0}%)", loaded_latency.download_avg_ms, loaded_latency.bufferbloat_download_ratio * 100.0);
    println!("  Upload Loaded: {:.1}ms (+{:.0}%)", loaded_latency.upload_avg_ms, loaded_latency.bufferbloat_upload_ratio * 100.0);
    println!("  Bufferbloat Grade: {}\n", loaded_latency.bufferbloat_grade.as_str());
    
    println!("AIM Scores:");
    println!("  Gaming: {:.0}/100", aim_scores.gaming.score);
    println!("  Streaming: {:.0}/100", aim_scores.streaming.score);
    println!("  Video Conferencing: {:.0}/100", aim_scores.video_conferencing.score);
    println!("  General Browsing: {:.0}/100\n", aim_scores.general_browsing.score);
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    println!("🤖 Analyzing with AI...\n");
    println!("This may take 10-20 seconds as GPT-4o-mini analyzes your results...\n");
    
    // Generate AI insights
    match ai_analyzer.analyze_network(&test_result, &loaded_latency, &aim_scores).await {
        Ok(insights) => {
            println!("✅ AI Analysis Complete!\n");
            println!("{}", insights.display());
            
            // Also show JSON for API integration
            println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
            println!("📄 JSON Output (for API):\n");
            match serde_json::to_string_pretty(&insights) {
                Ok(json) => println!("{}\n", json),
                Err(e) => println!("Error serializing: {}\n", e),
            }
        },
        Err(e) => {
            println!("❌ Error generating AI insights: {}", e);
            println!("\n💡 Possible issues:");
            println!("  • Invalid OpenAI API key");
            println!("  • API rate limit reached");
            println!("  • Network connection issue");
            println!("  • OpenAI service unavailable\n");
        }
    }
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ AI Insights Test Complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    println!("💡 Key Benefits of AI Integration:");
    println!("  • Natural language explanations anyone can understand");
    println!("  • Intelligent, personalized recommendations");
    println!("  • Predictive issue detection");
    println!("  • Context-aware troubleshooting");
    println!("  • Continuously improving with AI model updates\n");
}
