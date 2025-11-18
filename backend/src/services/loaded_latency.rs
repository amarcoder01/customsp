use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Loaded Latency Test - Measures latency in 3 stages
/// Based on research from Ookla and Cloudflare (2024)
/// 
/// This is THE most important metric for modern internet quality
/// because it reveals bufferbloat - the #1 cause of lag

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadedLatencyResult {
    // Stage 1: Idle latency (baseline)
    pub idle_min_ms: f64,
    pub idle_max_ms: f64,
    pub idle_avg_ms: f64,
    pub idle_median_ms: f64,
    pub idle_samples: Vec<f64>,
    
    // Stage 2: Download loaded latency
    pub download_min_ms: f64,
    pub download_max_ms: f64,
    pub download_avg_ms: f64,
    pub download_median_ms: f64,
    pub download_samples: Vec<f64>,
    
    // Stage 3: Upload loaded latency
    pub upload_min_ms: f64,
    pub upload_max_ms: f64,
    pub upload_avg_ms: f64,
    pub upload_median_ms: f64,
    pub upload_samples: Vec<f64>,
    
    // Derived metrics
    pub bufferbloat_download_ms: f64,    // How much latency increased
    pub bufferbloat_upload_ms: f64,
    pub bufferbloat_download_ratio: f64, // Percentage increase
    pub bufferbloat_upload_ratio: f64,
    pub bufferbloat_grade: BufferbloatGrade,
    
    // Responsiveness (Apple RPM metric)
    pub idle_rpm: f64,
    pub download_rpm: f64,
    pub upload_rpm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BufferbloatGrade {
    #[serde(rename = "A+")]
    APlus,      // <50% increase (excellent)
    A,          // <100% increase (very good)
    B,          // <200% increase (good)
    C,          // <400% increase (fair)
    D,          // <900% increase (poor)
    F,          // >900% increase (terrible)
}

impl BufferbloatGrade {
    pub fn as_str(&self) -> &str {
        match self {
            BufferbloatGrade::APlus => "A+",
            BufferbloatGrade::A => "A",
            BufferbloatGrade::B => "B",
            BufferbloatGrade::C => "C",
            BufferbloatGrade::D => "D",
            BufferbloatGrade::F => "F",
        }
    }
    
    pub fn description(&self) -> &str {
        match self {
            BufferbloatGrade::APlus => "Excellent - No bufferbloat detected",
            BufferbloatGrade::A => "Very Good - Minimal bufferbloat",
            BufferbloatGrade::B => "Good - Some bufferbloat, usually acceptable",
            BufferbloatGrade::C => "Fair - Noticeable bufferbloat, may cause lag",
            BufferbloatGrade::D => "Poor - Significant bufferbloat, expect lag spikes",
            BufferbloatGrade::F => "Terrible - Severe bufferbloat, gaming/video calls affected",
        }
    }
    
    pub fn emoji(&self) -> &str {
        match self {
            BufferbloatGrade::APlus => "⭐",
            BufferbloatGrade::A => "✅",
            BufferbloatGrade::B => "✓",
            BufferbloatGrade::C => "⚠️",
            BufferbloatGrade::D => "🔴",
            BufferbloatGrade::F => "❌",
        }
    }
}

pub struct LoadedLatencyTester {
    idle_pings: Vec<f64>,
    download_pings: Vec<f64>,
    upload_pings: Vec<f64>,
}

impl LoadedLatencyTester {
    pub fn new() -> Self {
        Self {
            idle_pings: Vec::new(),
            download_pings: Vec::new(),
            upload_pings: Vec::new(),
        }
    }
    
    /// Stage 1: Measure idle latency (baseline)
    /// This should be done BEFORE any data transfer
    pub async fn measure_idle_latency(&mut self, target: &str, samples: usize) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("📊 Stage 1: Measuring idle latency ({} samples)...", samples);
        
        for i in 0..samples {
            let latency = self.ping_once(target).await?;
            self.idle_pings.push(latency);
            
            if i % 5 == 0 {
                log::debug!("Idle ping {}/{}: {:.2}ms", i + 1, samples, latency);
            }
            
            // Small delay between pings (100ms)
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        
        let avg = self.calculate_average(&self.idle_pings);
        log::info!("✅ Idle latency baseline: {:.2}ms", avg);
        
        Ok(())
    }
    
    /// Stage 2: Measure latency DURING download
    /// Call this repeatedly while download test is running
    pub async fn measure_download_loaded_latency(&mut self, target: &str) -> Result<f64, Box<dyn std::error::Error>> {
        let latency = self.ping_once(target).await?;
        self.download_pings.push(latency);
        Ok(latency)
    }
    
    /// Stage 3: Measure latency DURING upload
    /// Call this repeatedly while upload test is running
    pub async fn measure_upload_loaded_latency(&mut self, target: &str) -> Result<f64, Box<dyn std::error::Error>> {
        let latency = self.ping_once(target).await?;
        self.upload_pings.push(latency);
        Ok(latency)
    }
    
    /// Perform a single ping measurement
    /// Uses HTTP GET as a "ping" - measures round-trip time
    async fn ping_once(&self, target: &str) -> Result<f64, Box<dyn std::error::Error>> {
        let start = Instant::now();
        
        // Simple HTTP GET to measure RTT
        // In production, this would be more sophisticated
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()?;
        
        let url = format!("http://{}/api/health", target);
        let _ = client.get(&url).send().await?;
        
        let duration = start.elapsed();
        let latency_ms = duration.as_secs_f64() * 1000.0;
        
        Ok(latency_ms)
    }
    
    /// Calculate final results and bufferbloat grade
    pub fn calculate_results(&self) -> LoadedLatencyResult {
        log::info!("📊 Calculating loaded latency results...");
        
        // Calculate stats for each stage
        let idle_stats = self.calculate_stats(&self.idle_pings);
        let download_stats = self.calculate_stats(&self.download_pings);
        let upload_stats = self.calculate_stats(&self.upload_pings);
        
        // Calculate bufferbloat
        let bufferbloat_download_ms = download_stats.avg - idle_stats.avg;
        let bufferbloat_upload_ms = upload_stats.avg - idle_stats.avg;
        
        let bufferbloat_download_ratio = if idle_stats.avg > 0.0 {
            bufferbloat_download_ms / idle_stats.avg
        } else {
            0.0
        };
        
        let bufferbloat_upload_ratio = if idle_stats.avg > 0.0 {
            bufferbloat_upload_ms / idle_stats.avg
        } else {
            0.0
        };
        
        // Worst-case bufferbloat determines the grade
        let worst_ratio = f64::max(bufferbloat_download_ratio, bufferbloat_upload_ratio);
        let bufferbloat_grade = Self::calculate_bufferbloat_grade(worst_ratio);
        
        // Calculate RPM (Responsiveness Per Minute) - Apple's metric
        let idle_rpm = Self::calculate_rpm(idle_stats.avg);
        let download_rpm = Self::calculate_rpm(download_stats.avg);
        let upload_rpm = Self::calculate_rpm(upload_stats.avg);
        
        log::info!("✅ Results calculated:");
        log::info!("   Idle: {:.2}ms ({:.0} RPM)", idle_stats.avg, idle_rpm);
        log::info!("   Download: {:.2}ms ({:.0} RPM) - +{:.0}%", 
            download_stats.avg, download_rpm, bufferbloat_download_ratio * 100.0);
        log::info!("   Upload: {:.2}ms ({:.0} RPM) - +{:.0}%", 
            upload_stats.avg, upload_rpm, bufferbloat_upload_ratio * 100.0);
        log::info!("   Bufferbloat Grade: {} {}", 
            bufferbloat_grade.emoji(), bufferbloat_grade.as_str());
        
        LoadedLatencyResult {
            // Idle
            idle_min_ms: idle_stats.min,
            idle_max_ms: idle_stats.max,
            idle_avg_ms: idle_stats.avg,
            idle_median_ms: idle_stats.median,
            idle_samples: self.idle_pings.clone(),
            
            // Download loaded
            download_min_ms: download_stats.min,
            download_max_ms: download_stats.max,
            download_avg_ms: download_stats.avg,
            download_median_ms: download_stats.median,
            download_samples: self.download_pings.clone(),
            
            // Upload loaded
            upload_min_ms: upload_stats.min,
            upload_max_ms: upload_stats.max,
            upload_avg_ms: upload_stats.avg,
            upload_median_ms: upload_stats.median,
            upload_samples: self.upload_pings.clone(),
            
            // Bufferbloat
            bufferbloat_download_ms,
            bufferbloat_upload_ms,
            bufferbloat_download_ratio,
            bufferbloat_upload_ratio,
            bufferbloat_grade,
            
            // RPM
            idle_rpm,
            download_rpm,
            upload_rpm,
        }
    }
    
    /// Calculate bufferbloat grade based on latency increase ratio
    fn calculate_bufferbloat_grade(ratio: f64) -> BufferbloatGrade {
        match ratio {
            r if r < 0.5 => BufferbloatGrade::APlus,   // <50% increase
            r if r < 1.0 => BufferbloatGrade::A,       // <100% increase
            r if r < 2.0 => BufferbloatGrade::B,       // <200% increase
            r if r < 4.0 => BufferbloatGrade::C,       // <400% increase
            r if r < 9.0 => BufferbloatGrade::D,       // <900% increase
            _ => BufferbloatGrade::F,                   // >900% increase
        }
    }
    
    /// Calculate RPM (Roundtrips Per Minute) - Apple's metric
    /// Higher = better responsiveness
    fn calculate_rpm(latency_ms: f64) -> f64 {
        if latency_ms > 0.0 {
            60_000.0 / latency_ms
        } else {
            0.0
        }
    }
    
    /// Calculate statistics for a set of measurements
    fn calculate_stats(&self, samples: &[f64]) -> LatencyStats {
        if samples.is_empty() {
            return LatencyStats {
                min: 0.0,
                max: 0.0,
                avg: 0.0,
                median: 0.0,
            };
        }
        
        let mut sorted = samples.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let min = *sorted.first().unwrap();
        let max = *sorted.last().unwrap();
        let avg = samples.iter().sum::<f64>() / samples.len() as f64;
        let median = if sorted.len() % 2 == 0 {
            (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
        } else {
            sorted[sorted.len() / 2]
        };
        
        LatencyStats {
            min,
            max,
            avg,
            median,
        }
    }
    
    fn calculate_average(&self, samples: &[f64]) -> f64 {
        if samples.is_empty() {
            return 0.0;
        }
        samples.iter().sum::<f64>() / samples.len() as f64
    }
}

#[derive(Debug)]
struct LatencyStats {
    min: f64,
    max: f64,
    avg: f64,
    median: f64,
}

impl LoadedLatencyResult {
    /// Get a human-readable summary of the test
    pub fn summary(&self) -> String {
        format!(
            "Loaded Latency Test Results\n\
             ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
             📊 Idle:     {:.1}ms ({:.0} RPM) {}\n\
             📥 Download: {:.1}ms ({:.0} RPM) {}\n\
             📤 Upload:   {:.1}ms ({:.0} RPM) {}\n\
             ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
             {} Bufferbloat Grade: {} - {}\n\
             Download increase: +{:.0}% ({:.1}ms)\n\
             Upload increase: +{:.0}% ({:.1}ms)",
            self.idle_avg_ms, self.idle_rpm, "⭐",
            self.download_avg_ms, self.download_rpm,
            if self.bufferbloat_download_ratio < 1.0 { "✅" } else { "⚠️" },
            self.upload_avg_ms, self.upload_rpm,
            if self.bufferbloat_upload_ratio < 1.0 { "✅" } else { "⚠️" },
            self.bufferbloat_grade.emoji(),
            self.bufferbloat_grade.as_str(),
            self.bufferbloat_grade.description(),
            self.bufferbloat_download_ratio * 100.0,
            self.bufferbloat_download_ms,
            self.bufferbloat_upload_ratio * 100.0,
            self.bufferbloat_upload_ms
        )
    }
    
    /// Get fix recommendations based on bufferbloat grade
    pub fn recommendations(&self) -> Vec<String> {
        match self.bufferbloat_grade {
            BufferbloatGrade::APlus | BufferbloatGrade::A => {
                vec![
                    "✅ Your connection has excellent quality!".to_string(),
                    "No bufferbloat detected - latency stays low under load.".to_string(),
                ]
            },
            BufferbloatGrade::B => {
                vec![
                    "✓ Your connection quality is good.".to_string(),
                    "Minor bufferbloat detected, but should not affect most activities.".to_string(),
                ]
            },
            BufferbloatGrade::C => {
                vec![
                    "⚠️ Moderate bufferbloat detected.".to_string(),
                    "May cause lag in gaming or choppy video calls during uploads/downloads.".to_string(),
                    "Consider enabling Smart Queue Management (SQM/QoS) on your router.".to_string(),
                ]
            },
            BufferbloatGrade::D | BufferbloatGrade::F => {
                vec![
                    "🔴 Significant bufferbloat detected!".to_string(),
                    "This will cause lag spikes, frozen video calls, and poor gaming experience.".to_string(),
                    "".to_string(),
                    "How to fix:".to_string(),
                    "1. Enable Smart Queue Management (SQM/QoS) in your router settings".to_string(),
                    "2. Set upload/download limits to 85-90% of your maximum speed".to_string(),
                    "3. Consider upgrading to a router with better bufferbloat mitigation".to_string(),
                    "4. Learn more at: https://www.bufferbloat.net".to_string(),
                ]
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bufferbloat_grading() {
        assert_eq!(LoadedLatencyTester::calculate_bufferbloat_grade(0.3), BufferbloatGrade::APlus);
        assert_eq!(LoadedLatencyTester::calculate_bufferbloat_grade(0.8), BufferbloatGrade::A);
        assert_eq!(LoadedLatencyTester::calculate_bufferbloat_grade(1.5), BufferbloatGrade::B);
        assert_eq!(LoadedLatencyTester::calculate_bufferbloat_grade(3.0), BufferbloatGrade::C);
        assert_eq!(LoadedLatencyTester::calculate_bufferbloat_grade(5.0), BufferbloatGrade::D);
        assert_eq!(LoadedLatencyTester::calculate_bufferbloat_grade(10.0), BufferbloatGrade::F);
    }
    
    #[test]
    fn test_rpm_calculation() {
        assert!((LoadedLatencyTester::calculate_rpm(10.0) - 6000.0).abs() < 0.1);
        assert!((LoadedLatencyTester::calculate_rpm(50.0) - 1200.0).abs() < 0.1);
        assert!((LoadedLatencyTester::calculate_rpm(100.0) - 600.0).abs() < 0.1);
    }
}
