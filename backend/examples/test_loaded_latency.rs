/// Test program for Loaded Latency Testing
/// 
/// This demonstrates the most critical improvement from research:
/// 3-stage latency measurement that reveals bufferbloat
///
/// Run with: cargo run --example test_loaded_latency

use speedtest_pro_backend::services::loaded_latency::{LoadedLatencyTester, BufferbloatGrade};

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    println!("\n🚀 SpeedTestPro - Loaded Latency Test");
    println!("=====================================\n");
    println!("This test measures latency in 3 stages:");
    println!("1. Idle latency (baseline)");
    println!("2. Latency during download");
    println!("3. Latency during upload\n");
    println!("This reveals 'bufferbloat' - the #1 cause of lag!\n");
    
    // Create tester
    let mut tester = LoadedLatencyTester::new();
    
    // For this example, we'll test against localhost
    // In production, this would be the actual test server
    let target = "127.0.0.1:8080";
    
    println!("Target: {}\n", target);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // STAGE 1: Measure idle latency
    println!("📊 STAGE 1: Measuring Idle Latency");
    println!("   (Network at rest, no data transfer)");
    println!("   Sending 20 pings...\n");
    
    match tester.measure_idle_latency(target, 20).await {
        Ok(_) => println!("   ✅ Idle baseline established\n"),
        Err(e) => {
            println!("   ❌ Error: {}", e);
            println!("\n💡 Make sure the backend server is running:");
            println!("   cd backend && cargo run\n");
            return;
        }
    }
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // STAGE 2: Simulate download test with loaded latency measurement
    println!("📊 STAGE 2: Measuring Download Loaded Latency");
    println!("   (Latency DURING download test)");
    println!("   Simulating download for 10 seconds...\n");
    
    let download_duration = tokio::time::Duration::from_secs(10);
    let start = tokio::time::Instant::now();
    
    while start.elapsed() < download_duration {
        // Measure latency every 500ms during download
        match tester.measure_download_loaded_latency(target).await {
            Ok(latency) => {
                if start.elapsed().as_secs() % 2 == 0 {
                    println!("   Download ping: {:.2}ms", latency);
                }
            },
            Err(e) => {
                println!("   ⚠️  Warning: {}", e);
                break;
            }
        }
        
        // Wait before next ping
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    
    println!("\n   ✅ Download loaded latency measured\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // STAGE 3: Simulate upload test with loaded latency measurement
    println!("📊 STAGE 3: Measuring Upload Loaded Latency");
    println!("   (Latency DURING upload test)");
    println!("   Simulating upload for 10 seconds...\n");
    
    let upload_duration = tokio::time::Duration::from_secs(10);
    let start = tokio::time::Instant::now();
    
    while start.elapsed() < upload_duration {
        // Measure latency every 500ms during upload
        match tester.measure_upload_loaded_latency(target).await {
            Ok(latency) => {
                if start.elapsed().as_secs() % 2 == 0 {
                    println!("   Upload ping: {:.2}ms", latency);
                }
            },
            Err(e) => {
                println!("   ⚠️  Warning: {}", e);
                break;
            }
        }
        
        // Wait before next ping
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    
    println!("\n   ✅ Upload loaded latency measured\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // Calculate final results
    println!("📊 Calculating Results...\n");
    let results = tester.calculate_results();
    
    // Display results
    println!("{}\n", results.summary());
    
    // Display grade explanation
    println!("\n💡 What does this mean?\n");
    for recommendation in results.recommendations() {
        println!("   {}", recommendation);
    }
    
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // Export results as JSON
    println!("📄 JSON Results:\n");
    match serde_json::to_string_pretty(&results) {
        Ok(json) => println!("{}\n", json),
        Err(e) => println!("   Error serializing results: {}\n", e),
    }
    
    // Interpret the grade
    println!("🎯 Your Bufferbloat Grade: {} {}", 
        results.bufferbloat_grade.emoji(), 
        results.bufferbloat_grade.as_str()
    );
    println!("   {}\n", results.bufferbloat_grade.description());
    
    // Grade-specific advice
    match results.bufferbloat_grade {
        BufferbloatGrade::APlus | BufferbloatGrade::A => {
            println!("🎉 Excellent! Your network has minimal bufferbloat.");
            println!("   Perfect for gaming, video calls, and streaming.\n");
        },
        BufferbloatGrade::B => {
            println!("👍 Good! Minor bufferbloat detected.");
            println!("   Most activities should work well.\n");
        },
        BufferbloatGrade::C => {
            println!("⚠️  Fair. Moderate bufferbloat may cause issues.");
            println!("   Consider enabling SQM/QoS on your router.\n");
        },
        BufferbloatGrade::D | BufferbloatGrade::F => {
            println!("🔴 Poor! Significant bufferbloat detected.");
            println!("   This WILL cause lag in games and video calls.");
            println!("   Fix: Enable Smart Queue Management (SQM).\n");
        },
    }
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Test Complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}
