/// AIM (Aggregated Internet Measurement) Scoring System
/// 
/// Based on Cloudflare's approach to translate raw metrics into
/// use-case specific quality scores that users can understand.
/// 
/// Instead of showing raw numbers, we answer:
/// - "Is my internet good for gaming?"
/// - "Can I stream 4K video?"
/// - "Will video calls work smoothly?"

use serde::{Deserialize, Serialize};
use crate::models::TestResult;
use crate::services::loaded_latency::LoadedLatencyResult;

/// Complete AIM scores for all use cases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMScores {
    pub gaming: UseCaseScore,
    pub streaming: UseCaseScore,
    pub video_conferencing: UseCaseScore,
    pub general_browsing: UseCaseScore,
    
    /// Overall quality score (0-100)
    pub overall_score: f64,
    pub overall_grade: QualityGrade,
}

/// Score for a specific use case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseCaseScore {
    /// Numerical score (0-100)
    pub score: f64,
    
    /// Letter grade
    pub grade: QualityGrade,
    
    /// Human-readable assessment
    pub assessment: String,
    
    /// Detailed explanation
    pub explanation: String,
    
    /// Emoji indicator
    pub emoji: String,
    
    /// What works / what doesn't
    pub capabilities: Vec<String>,
    
    /// Recommendations for improvement
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QualityGrade {
    Excellent,  // 90-100
    Good,       // 75-89
    Fair,       // 60-74
    Poor,       // 40-59
    VeryPoor,   // 0-39
}

impl QualityGrade {
    pub fn from_score(score: f64) -> Self {
        match score {
            s if s >= 90.0 => QualityGrade::Excellent,
            s if s >= 75.0 => QualityGrade::Good,
            s if s >= 60.0 => QualityGrade::Fair,
            s if s >= 40.0 => QualityGrade::Poor,
            _ => QualityGrade::VeryPoor,
        }
    }
    
    pub fn as_str(&self) -> &str {
        match self {
            QualityGrade::Excellent => "Excellent",
            QualityGrade::Good => "Good",
            QualityGrade::Fair => "Fair",
            QualityGrade::Poor => "Poor",
            QualityGrade::VeryPoor => "Very Poor",
        }
    }
    
    pub fn emoji(&self) -> &str {
        match self {
            QualityGrade::Excellent => "⭐",
            QualityGrade::Good => "✅",
            QualityGrade::Fair => "✓",
            QualityGrade::Poor => "⚠️",
            QualityGrade::VeryPoor => "❌",
        }
    }
}

pub struct AIMCalculator;

impl AIMCalculator {
    /// Calculate all AIM scores from test results
    pub fn calculate_all_scores(
        test_result: &TestResult,
        loaded_latency: &LoadedLatencyResult,
    ) -> AIMScores {
        log::info!("📊 Calculating AIM scores for all use cases...");
        
        let gaming = Self::calculate_gaming_score(test_result, loaded_latency);
        let streaming = Self::calculate_streaming_score(test_result, loaded_latency);
        let video_conferencing = Self::calculate_conferencing_score(test_result, loaded_latency);
        let general_browsing = Self::calculate_browsing_score(test_result, loaded_latency);
        
        // Overall score is weighted average
        let overall_score = (
            gaming.score * 0.25 +
            streaming.score * 0.25 +
            video_conferencing.score * 0.25 +
            general_browsing.score * 0.25
        );
        
        let overall_grade = QualityGrade::from_score(overall_score);
        
        log::info!("✅ AIM scores calculated - Overall: {:.0}/100 ({})", 
            overall_score, overall_grade.as_str());
        
        AIMScores {
            gaming,
            streaming,
            video_conferencing,
            general_browsing,
            overall_score,
            overall_grade,
        }
    }
    
    /// Gaming Score - Latency and jitter are MOST important
    /// 
    /// Weights:
    /// - Loaded latency: 50% (critical for responsiveness)
    /// - Jitter: 25% (causes inconsistent gameplay)
    /// - Packet loss: 15% (causes rubber-banding)
    /// - Download speed: 10% (less critical - games use <10 Mbps)
    pub fn calculate_gaming_score(
        test_result: &TestResult,
        loaded_latency: &LoadedLatencyResult,
    ) -> UseCaseScore {
        let mut score = 100.0;
        let mut issues = Vec::new();
        let mut capabilities = Vec::new();
        let mut recommendations = Vec::new();
        
        // 1. Loaded Latency (50 points) - CRITICAL for gaming
        let worst_latency = f64::max(
            loaded_latency.download_avg_ms,
            loaded_latency.upload_avg_ms
        );
        
        let latency_score = match worst_latency {
            l if l < 20.0 => {
                capabilities.push("Perfect for competitive gaming (esports-level)".to_string());
                50.0
            },
            l if l < 50.0 => {
                capabilities.push("Excellent for online gaming".to_string());
                45.0
            },
            l if l < 80.0 => {
                capabilities.push("Good for most online games".to_string());
                issues.push(format!("Latency {}ms - noticeable in fast-paced games", l as u32));
                35.0
            },
            l if l < 100.0 => {
                capabilities.push("Playable for casual games".to_string());
                issues.push(format!("Latency {}ms - not ideal for competitive play", l as u32));
                recommendations.push("Reduce bufferbloat to improve latency".to_string());
                25.0
            },
            l if l < 150.0 => {
                issues.push(format!("High latency {}ms - lag will be noticeable", l as u32));
                recommendations.push("Enable SQM/QoS on router".to_string());
                recommendations.push("Consider wired connection instead of WiFi".to_string());
                15.0
            },
            l => {
                issues.push(format!("Very high latency {}ms - gaming will be frustrating", l as u32));
                recommendations.push("Check for network congestion".to_string());
                recommendations.push("Contact ISP about high latency".to_string());
                5.0
            }
        };
        score = latency_score;
        
        // 2. Jitter (25 points)
        let jitter_score = match test_result.jitter_ms {
            j if j < 5.0 => {
                capabilities.push("Consistent performance - no lag spikes".to_string());
                25.0
            },
            j if j < 15.0 => {
                20.0
            },
            j if j < 30.0 => {
                issues.push(format!("Jitter {}ms - occasional lag spikes", j as u32));
                15.0
            },
            j => {
                issues.push(format!("High jitter {}ms - frequent lag spikes", j as u32));
                recommendations.push("Check WiFi signal strength".to_string());
                5.0
            }
        };
        score += jitter_score;
        
        // 3. Packet Loss (15 points)
        // For now, we'll assume 0% - will implement in Phase 3
        let packet_loss_pct = 0.0;
        let packet_loss_score = match packet_loss_pct {
            p if p < 0.1 => 15.0,
            p if p < 1.0 => 10.0,
            p if p < 3.0 => {
                issues.push(format!("Packet loss {:.1}% - rubber-banding may occur", p));
                5.0
            },
            p => {
                issues.push(format!("High packet loss {:.1}% - game will be unplayable", p));
                0.0
            }
        };
        score += packet_loss_score;
        
        // 4. Download Speed (10 points) - least important for gaming
        let speed_score = match test_result.download_mbps {
            s if s >= 25.0 => {
                10.0
            },
            s if s >= 10.0 => {
                8.0
            },
            s if s >= 5.0 => {
                issues.push("Low bandwidth may cause download delays".to_string());
                5.0
            },
            _ => {
                issues.push("Very low bandwidth - game downloads will be slow".to_string());
                2.0
            }
        };
        score += speed_score;
        
        let grade = QualityGrade::from_score(score);
        
        let assessment = match grade {
            QualityGrade::Excellent => "Perfect for competitive gaming - esports ready!",
            QualityGrade::Good => "Great for online gaming - smooth experience",
            QualityGrade::Fair => "Playable but not ideal - casual gaming okay",
            QualityGrade::Poor => "Poor gaming experience - lag will be noticeable",
            QualityGrade::VeryPoor => "Not suitable for online gaming",
        }.to_string();
        
        let explanation = format!(
            "Gaming requires low latency ({:.0}ms) and stable connection. {}",
            worst_latency,
            if worst_latency < 50.0 { "Your connection is excellent for gaming." }
            else if worst_latency < 100.0 { "Your connection is acceptable but could be better." }
            else { "High latency will cause noticeable lag." }
        );
        
        let emoji = Self::gaming_emoji(&grade);

        UseCaseScore {
            score,
            grade,
            assessment,
            explanation,
            emoji,
            capabilities,
            recommendations,
        }
    }
    
    /// Streaming Score - Download speed and consistency matter most
    /// 
    /// Weights:
    /// - Download speed: 40%
    /// - Download loaded latency: 30% (buffering prevention)
    /// - Jitter: 20% (stream stability)
    /// - Packet loss: 10%
    pub fn calculate_streaming_score(
        test_result: &TestResult,
        loaded_latency: &LoadedLatencyResult,
    ) -> UseCaseScore {
        let mut score = 100.0;
        let mut capabilities = Vec::new();
        let mut recommendations = Vec::new();
        
        // 1. Download Speed (40 points)
        let speed_score = match test_result.download_mbps {
            s if s >= 100.0 => {
                capabilities.push("8K streaming on multiple devices".to_string());
                capabilities.push("4K 60fps streaming with headroom".to_string());
                40.0
            },
            s if s >= 50.0 => {
                capabilities.push("4K streaming on 2-3 devices".to_string());
                capabilities.push("HD streaming on many devices".to_string());
                38.0
            },
            s if s >= 25.0 => {
                capabilities.push("4K streaming on 1 device".to_string());
                capabilities.push("HD streaming on 2-3 devices".to_string());
                35.0
            },
            s if s >= 15.0 => {
                capabilities.push("HD (1080p) streaming reliably".to_string());
                30.0
            },
            s if s >= 10.0 => {
                capabilities.push("HD streaming on 1 device".to_string());
                recommendations.push("4K may buffer occasionally".to_string());
                25.0
            },
            s if s >= 5.0 => {
                capabilities.push("SD/HD streaming works".to_string());
                recommendations.push("Avoid 4K streaming".to_string());
                15.0
            },
            s => {
                recommendations.push(format!("Speed {:.1} Mbps too low for HD", s));
                recommendations.push("Upgrade plan for better streaming".to_string());
                5.0
            }
        };
        score = speed_score;
        
        // 2. Download Loaded Latency (30 points)
        let latency_score = match loaded_latency.download_avg_ms {
            l if l < 50.0 => 30.0,
            l if l < 100.0 => 25.0,
            l if l < 200.0 => {
                recommendations.push("High latency may cause buffering".to_string());
                20.0
            },
            _ => {
                recommendations.push("Reduce bufferbloat for smoother streaming".to_string());
                10.0
            }
        };
        score += latency_score;
        
        // 3. Jitter (20 points)
        let jitter_score = match test_result.jitter_ms {
            j if j < 10.0 => 20.0,
            j if j < 30.0 => 15.0,
            j if j < 50.0 => 10.0,
            _ => 5.0,
        };
        score += jitter_score;
        
        // 4. Packet Loss (10 points)
        score += 10.0; // Assume good for now
        
        let grade = QualityGrade::from_score(score);
        
        let assessment = match grade {
            QualityGrade::Excellent => "Perfect for 4K/8K streaming on multiple devices",
            QualityGrade::Good => "Great for 4K streaming and HD on multiple devices",
            QualityGrade::Fair => "HD streaming works, 4K may buffer occasionally",
            QualityGrade::Poor => "SD/HD only, frequent buffering possible",
            QualityGrade::VeryPoor => "Streaming will be problematic",
        }.to_string();
        
        let explanation = format!(
            "Streaming quality depends on download speed ({:.1} Mbps) and stability. {}",
            test_result.download_mbps,
            if test_result.download_mbps >= 25.0 { "Your speed is excellent for streaming." }
            else if test_result.download_mbps >= 10.0 { "Your speed is adequate for HD streaming." }
            else { "Your speed may struggle with HD content." }
        );
        
        UseCaseScore {
            score,
            grade,
            assessment,
            explanation,
            emoji: "📺".to_string(),
            capabilities,
            recommendations,
        }
    }
    
    /// Video Conferencing Score - Upload and symmetry matter
    /// 
    /// Weights:
    /// - Upload speed: 30%
    /// - Upload loaded latency: 30% (critical - frozen video!)
    /// - Jitter: 25%
    /// - Download speed: 15%
    pub fn calculate_conferencing_score(
        test_result: &TestResult,
        loaded_latency: &LoadedLatencyResult,
    ) -> UseCaseScore {
        let mut score = 100.0;
        let mut capabilities = Vec::new();
        let mut recommendations = Vec::new();
        
        // 1. Upload Speed (30 points)
        let upload_score = match test_result.upload_mbps {
            u if u >= 20.0 => {
                capabilities.push("4K video calls with screen sharing".to_string());
                30.0
            },
            u if u >= 10.0 => {
                capabilities.push("HD video calls with screen sharing".to_string());
                28.0
            },
            u if u >= 5.0 => {
                capabilities.push("HD video calls work well".to_string());
                25.0
            },
            u if u >= 3.0 => {
                capabilities.push("HD video calls (may struggle with screen share)".to_string());
                20.0
            },
            u if u >= 1.5 => {
                capabilities.push("SD video calls work".to_string());
                recommendations.push("HD may be choppy".to_string());
                15.0
            },
            u => {
                recommendations.push(format!("Upload {:.1} Mbps too low for video", u));
                recommendations.push("Use audio-only or upgrade plan".to_string());
                5.0
            }
        };
        score = upload_score;
        
        // 2. Upload Loaded Latency (30 points) - CRITICAL
        let latency_score = match loaded_latency.upload_avg_ms {
            l if l < 30.0 => {
                capabilities.push("Smooth real-time conversation".to_string());
                30.0
            },
            l if l < 80.0 => {
                25.0
            },
            l if l < 150.0 => {
                recommendations.push("Latency may cause awkward pauses".to_string());
                20.0
            },
            l if l < 250.0 => {
                recommendations.push(format!("High upload latency {}ms", l as u32));
                recommendations.push("Enable SQM to reduce bufferbloat".to_string());
                10.0
            },
            l => {
                recommendations.push(format!("Very high upload latency {}ms", l as u32));
                recommendations.push("Video will freeze frequently".to_string());
                5.0
            }
        };
        score += latency_score;
        
        // 3. Jitter (25 points)
        let jitter_score = match test_result.jitter_ms {
            j if j < 10.0 => 25.0,
            j if j < 20.0 => 20.0,
            j if j < 40.0 => {
                recommendations.push("Jitter may cause choppy audio/video".to_string());
                15.0
            },
            _ => 5.0,
        };
        score += jitter_score;
        
        // 4. Download Speed (15 points)
        let download_score = match test_result.download_mbps {
            d if d >= 10.0 => 15.0,
            d if d >= 5.0 => 12.0,
            d if d >= 2.5 => 8.0,
            _ => 3.0,
        };
        score += download_score;
        
        let grade = QualityGrade::from_score(score);
        
        let assessment = match grade {
            QualityGrade::Excellent => "Perfect for 4K video calls and screen sharing",
            QualityGrade::Good => "HD video conferencing works smoothly",
            QualityGrade::Fair => "Video calls work, occasional quality drops",
            QualityGrade::Poor => "Video calls may be choppy or freeze",
            QualityGrade::VeryPoor => "Not suitable for video conferencing",
        }.to_string();
        
        let explanation = format!(
            "Video calls need good upload ({:.1} Mbps) and low upload latency ({:.0}ms). {}",
            test_result.upload_mbps,
            loaded_latency.upload_avg_ms,
            if loaded_latency.upload_avg_ms < 80.0 && test_result.upload_mbps >= 5.0 {
                "Your connection is great for video calls."
            } else if loaded_latency.upload_avg_ms > 150.0 {
                "High upload latency will cause frozen video."
            } else {
                "Your connection should work for video calls."
            }
        );
        
        UseCaseScore {
            score,
            grade,
            assessment,
            explanation,
            emoji: "💼".to_string(),
            capabilities,
            recommendations,
        }
    }
    
    /// General Browsing Score - Overall balance
    /// 
    /// Weights:
    /// - Download speed: 40%
    /// - Idle latency: 40%
    /// - Jitter: 10%
    /// - Packet loss: 10%
    pub fn calculate_browsing_score(
        test_result: &TestResult,
        loaded_latency: &LoadedLatencyResult,
    ) -> UseCaseScore {
        let mut score = 100.0;
        let mut capabilities = Vec::new();
        let mut recommendations = Vec::new();
        
        // 1. Download Speed (40 points)
        let speed_score = match test_result.download_mbps {
            s if s >= 100.0 => {
                capabilities.push("Lightning-fast page loads".to_string());
                capabilities.push("Instant large downloads".to_string());
                40.0
            },
            s if s >= 50.0 => {
                capabilities.push("Very fast browsing experience".to_string());
                38.0
            },
            s if s >= 25.0 => {
                capabilities.push("Fast page loads and downloads".to_string());
                35.0
            },
            s if s >= 10.0 => {
                capabilities.push("Good browsing experience".to_string());
                30.0
            },
            s if s >= 5.0 => {
                capabilities.push("Adequate for basic browsing".to_string());
                20.0
            },
            s => {
                recommendations.push(format!("Speed {:.1} Mbps is slow", s));
                10.0
            }
        };
        score = speed_score;
        
        // 2. Idle Latency (40 points)
        let latency_score = match loaded_latency.idle_avg_ms {
            l if l < 20.0 => {
                capabilities.push("Instant page response".to_string());
                40.0
            },
            l if l < 50.0 => 35.0,
            l if l < 100.0 => 30.0,
            l if l < 200.0 => {
                recommendations.push("Pages may feel slightly sluggish".to_string());
                20.0
            },
            _ => 10.0,
        };
        score += latency_score;
        
        // 3. Jitter + Packet Loss (20 points)
        score += 20.0; // Assume good for now
        
        let grade = QualityGrade::from_score(score);
        
        let assessment = match grade {
            QualityGrade::Excellent => "Outstanding browsing experience - instant and smooth",
            QualityGrade::Good => "Great browsing - fast page loads",
            QualityGrade::Fair => "Adequate browsing - some delays",
            QualityGrade::Poor => "Slow browsing experience",
            QualityGrade::VeryPoor => "Very slow - frustrating to use",
        }.to_string();
        
        let explanation = format!(
            "Browsing quality combines speed ({:.1} Mbps) and responsiveness ({:.0}ms latency).",
            test_result.download_mbps,
            loaded_latency.idle_avg_ms
        );
        
        UseCaseScore {
            score,
            grade,
            assessment,
            explanation,
            emoji: "🌐".to_string(),
            capabilities,
            recommendations,
        }
    }
    
    fn gaming_emoji(grade: &QualityGrade) -> String {
        match grade {
            QualityGrade::Excellent => "🎮⭐",
            QualityGrade::Good => "🎮✅",
            QualityGrade::Fair => "🎮✓",
            QualityGrade::Poor => "🎮⚠️",
            QualityGrade::VeryPoor => "🎮❌",
        }.to_string()
    }
}

impl AIMScores {
    /// Get a formatted summary of all scores
    pub fn summary(&self) -> String {
        format!(
            "AIM Quality Scores\n\
             ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
             🎮 Gaming:            {:.0}/100 {} {}\n\
             📺 Streaming:         {:.0}/100 {} {}\n\
             💼 Video Conferencing: {:.0}/100 {} {}\n\
             🌐 General Browsing:   {:.0}/100 {} {}\n\
             ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
             Overall Quality: {:.0}/100 {} {}",
            self.gaming.score, self.gaming.grade.emoji(), self.gaming.grade.as_str(),
            self.streaming.score, self.streaming.grade.emoji(), self.streaming.grade.as_str(),
            self.video_conferencing.score, self.video_conferencing.grade.emoji(), self.video_conferencing.grade.as_str(),
            self.general_browsing.score, self.general_browsing.grade.emoji(), self.general_browsing.grade.as_str(),
            self.overall_score, self.overall_grade.emoji(), self.overall_grade.as_str()
        )
    }
    
    /// Get detailed report for a specific use case
    pub fn detailed_report(&self, use_case: &str) -> String {
        let score = match use_case.to_lowercase().as_str() {
            "gaming" => &self.gaming,
            "streaming" => &self.streaming,
            "conferencing" | "video" => &self.video_conferencing,
            "browsing" | "general" => &self.general_browsing,
            _ => return "Unknown use case".to_string(),
        };
        
        let mut report = format!(
            "{} {} - {:.0}/100 {}\n\
             ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
             {}\n\n\
             {}\n\n",
            score.emoji,
            use_case.to_uppercase(),
            score.score,
            score.grade.emoji(),
            score.assessment,
            score.explanation
        );
        
        if !score.capabilities.is_empty() {
            report.push_str("✅ What Works:\n");
            for cap in &score.capabilities {
                report.push_str(&format!("   • {}\n", cap));
            }
            report.push('\n');
        }
        
        if !score.recommendations.is_empty() {
            report.push_str("💡 Recommendations:\n");
            for rec in &score.recommendations {
                report.push_str(&format!("   • {}\n", rec));
            }
        }
        
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quality_grade_from_score() {
        assert_eq!(QualityGrade::from_score(95.0), QualityGrade::Excellent);
        assert_eq!(QualityGrade::from_score(80.0), QualityGrade::Good);
        assert_eq!(QualityGrade::from_score(65.0), QualityGrade::Fair);
        assert_eq!(QualityGrade::from_score(45.0), QualityGrade::Poor);
        assert_eq!(QualityGrade::from_score(20.0), QualityGrade::VeryPoor);
    }
}
