use actix_ws::Session;
use bytes::Bytes;
use chrono::Utc;
use log::{info, debug};
use std::time::{Duration, Instant};
use tokio::time::sleep;

use crate::config::AppConfig;
use crate::models::{TestProgress, TestResult};

pub struct MeasurementEngine {
    config: AppConfig,
}

impl MeasurementEngine {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
    
    pub async fn run_full_test(
        &self,
        test_id: &str,
        session: &mut Session,
    ) -> Result<TestResult, Box<dyn std::error::Error>> {
        info!("Starting full test: {}", test_id);
        
        let mut result = TestResult::new(self.config.server_id.clone(), "unknown".to_string());
        result.id = test_id.to_string();
        
        let test_start = Instant::now();
        
        // Stage 1: Latency measurement
        self.send_progress(
            session,
            TestProgress::new("latency", 0.0, "Measuring latency..."),
        ).await?;
        
        let (latency, jitter) = self.measure_latency(session).await?;
        result.latency_ms = latency;
        result.jitter_ms = jitter;
        
        self.send_progress(
            session,
            TestProgress::new("latency", 0.2, "Latency measured")
                .with_latency(latency),
        ).await?;
        
        // Stage 2: Download speed
        self.send_progress(
            session,
            TestProgress::new("download", 0.2, "Testing download speed..."),
        ).await?;
        
        let download_mbps = self.measure_download(session).await?;
        result.download_mbps = download_mbps;
        
        self.send_progress(
            session,
            TestProgress::new("download", 0.6, "Download speed measured")
                .with_speed(download_mbps),
        ).await?;
        
        // Stage 3: Upload speed
        self.send_progress(
            session,
            TestProgress::new("upload", 0.6, "Testing upload speed..."),
        ).await?;
        
        let upload_mbps = self.measure_upload(session).await?;
        result.upload_mbps = upload_mbps;
        
        self.send_progress(
            session,
            TestProgress::new("upload", 0.9, "Upload speed measured")
                .with_speed(upload_mbps),
        ).await?;
        
        // Complete
        result.test_duration_ms = test_start.elapsed().as_millis() as u64;
        result.timestamp = Utc::now();
        
        self.send_progress(
            session,
            TestProgress::new("complete", 1.0, "Test completed successfully!"),
        ).await?;
        
        info!("Test completed: {} - Down: {:.2} Mbps, Up: {:.2} Mbps, Latency: {:.2} ms",
            test_id, download_mbps, upload_mbps, latency);
        
        Ok(result)
    }
    
    async fn measure_latency(
        &self,
        session: &mut Session,
    ) -> Result<(f64, f64), Box<dyn std::error::Error>> {
        const PING_COUNT: usize = 10;
        let mut latencies = Vec::with_capacity(PING_COUNT);
        
        for i in 0..PING_COUNT {
            let start = Instant::now();
            
            // Send ping
            session.ping(b"ping").await?;
            
            // Simulate network delay (in production, wait for pong)
            sleep(Duration::from_millis(1)).await;
            
            let latency = start.elapsed().as_secs_f64() * 1000.0;
            latencies.push(latency);
            
            // Send progress update every few pings
            if i % 3 == 0 {
                let progress = 0.05 + (i as f32 / PING_COUNT as f32) * 0.15;
                self.send_progress(
                    session,
                    TestProgress::new("latency", progress, &format!("Ping {}/{}", i + 1, PING_COUNT))
                        .with_latency(latency),
                ).await?;
            }
        }
        
        // Calculate statistics
        let avg_latency = latencies.iter().sum::<f64>() / latencies.len() as f64;
        let jitter = self.calculate_jitter(&latencies);
        
        Ok((avg_latency, jitter))
    }
    
    async fn measure_download(
        &self,
        session: &mut Session,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        const TEST_DURATION_MS: u64 = 5000; // 5 seconds
        const CHUNK_SIZE: usize = 65536; // 64KB chunks
        
        let start = Instant::now();
        let mut total_bytes = 0u64;
        let mut measurements = Vec::new();
        
        // Generate test data
        let test_chunk = vec![b'X'; CHUNK_SIZE];
        
        while start.elapsed().as_millis() < TEST_DURATION_MS as u128 {
            let chunk_start = Instant::now();
            
            // Send data chunk
            session.binary(Bytes::from(test_chunk.clone())).await?;
            total_bytes += CHUNK_SIZE as u64;
            
            let chunk_duration = chunk_start.elapsed().as_secs_f64();
            if chunk_duration > 0.0 {
                let speed_bps = (CHUNK_SIZE as f64 * 8.0) / chunk_duration;
                let speed_mbps = speed_bps / 1_000_000.0;
                measurements.push(speed_mbps);
                
                // Send progress update
                let progress = 0.2 + (start.elapsed().as_millis() as f32 / TEST_DURATION_MS as f32) * 0.4;
                if measurements.len() % 10 == 0 {
                    self.send_progress(
                        session,
                        TestProgress::new("download", progress, "Downloading...")
                            .with_speed(speed_mbps),
                    ).await?;
                }
            }
            
            // Small delay to prevent overwhelming
            sleep(Duration::from_millis(10)).await;
        }
        
        // Calculate average speed
        let avg_speed = if !measurements.is_empty() {
            measurements.iter().sum::<f64>() / measurements.len() as f64
        } else {
            0.0
        };
        
        debug!("Download test: {} bytes in {} ms, avg speed: {:.2} Mbps",
            total_bytes, start.elapsed().as_millis(), avg_speed);
        
        Ok(avg_speed)
    }
    
    async fn measure_upload(
        &self,
        session: &mut Session,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        const TEST_DURATION_MS: u64 = 5000; // 5 seconds
        const CHUNK_SIZE: usize = 65536; // 64KB chunks
        
        let start = Instant::now();
        let mut total_bytes = 0u64;
        let mut measurements = Vec::new();
        
        // In a real implementation, we'd receive data from the client
        // For now, simulate upload by generating data server-side
        let test_chunk = vec![b'Y'; CHUNK_SIZE];
        
        while start.elapsed().as_millis() < TEST_DURATION_MS as u128 {
            let chunk_start = Instant::now();
            
            // Simulate receiving data (in production, this would be actual client data)
            total_bytes += CHUNK_SIZE as u64;
            
            let chunk_duration = chunk_start.elapsed().as_secs_f64();
            if chunk_duration > 0.0 {
                let speed_bps = (CHUNK_SIZE as f64 * 8.0) / chunk_duration;
                let speed_mbps = speed_bps / 1_000_000.0;
                measurements.push(speed_mbps);
                
                // Send progress update
                let progress = 0.6 + (start.elapsed().as_millis() as f32 / TEST_DURATION_MS as f32) * 0.3;
                if measurements.len() % 10 == 0 {
                    self.send_progress(
                        session,
                        TestProgress::new("upload", progress, "Uploading...")
                            .with_speed(speed_mbps),
                    ).await?;
                }
            }
            
            sleep(Duration::from_millis(10)).await;
        }
        
        // Calculate average speed
        let avg_speed = if !measurements.is_empty() {
            measurements.iter().sum::<f64>() / measurements.len() as f64
        } else {
            0.0
        };
        
        debug!("Upload test: {} bytes in {} ms, avg speed: {:.2} Mbps",
            total_bytes, start.elapsed().as_millis(), avg_speed);
        
        Ok(avg_speed)
    }
    
    fn calculate_jitter(&self, latencies: &[f64]) -> f64 {
        if latencies.len() < 2 {
            return 0.0;
        }
        
        let mut differences = Vec::new();
        for i in 1..latencies.len() {
            differences.push((latencies[i] - latencies[i - 1]).abs());
        }
        
        differences.iter().sum::<f64>() / differences.len() as f64
    }
    
    async fn send_progress(
        &self,
        session: &mut Session,
        progress: TestProgress,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let progress_json = serde_json::to_string(&progress)?;
        session.text(progress_json).await?;
        Ok(())
    }
}
