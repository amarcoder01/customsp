/// Packet Loss Detection using UDP
/// 
/// Measures packet loss by sending numbered UDP packets and tracking which are received.
/// This is more accurate than TCP which masks packet loss with retransmissions.

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;
use tokio::time::sleep;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketLossResult {
    pub sent_packets: u64,
    pub received_packets: u64,
    pub lost_packets: u64,
    pub loss_percentage: f64,
    pub grade: PacketLossGrade,
    pub test_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PacketLossGrade {
    Excellent, // <0.1%
    Good,      // <1.0%
    Fair,      // <3.0%
    Poor,      // >=3.0%
}

pub struct PacketLossDetector {
    packets_per_second: u32,
    test_duration_seconds: u32,
}

impl PacketLossDetector {
    pub fn new() -> Self {
        Self {
            packets_per_second: 50,  // Industry standard
            test_duration_seconds: 10,
        }
    }

    /// Measure packet loss using UDP echo
    pub async fn measure_packet_loss(&self, target: &str) -> Result<PacketLossResult, String> {
        let start_time = Instant::now();
        
        // Create UDP socket
        let socket = UdpSocket::bind("0.0.0.0:0")
            .await
            .map_err(|e| format!("Failed to bind UDP socket: {}", e))?;
        
        socket.connect(target)
            .await
            .map_err(|e| format!("Failed to connect to target: {}", e))?;

        let total_packets = self.packets_per_second * self.test_duration_seconds;
        let mut sent_packets = 0u64;
        let mut received_packets = 0u64;
        
        let interval = Duration::from_millis(1000 / self.packets_per_second as u64);
        
        // Send packets
        for sequence in 0..total_packets {
            // Create packet with sequence number
            let packet = format!("SPEEDTEST_PACKET:{}", sequence);
            
            match socket.send(packet.as_bytes()).await {
                Ok(_) => {
                    sent_packets += 1;
                    
                    // Try to receive echo (with timeout)
                    let mut buf = [0u8; 1024];
                    match tokio::time::timeout(
                        Duration::from_millis(100),
                        socket.recv(&mut buf)
                    ).await {
                        Ok(Ok(_)) => received_packets += 1,
                        _ => {} // Packet lost or timeout
                    }
                }
                Err(e) => {
                    log::warn!("Failed to send packet {}: {}", sequence, e);
                }
            }
            
            sleep(interval).await;
        }
        
        let elapsed = start_time.elapsed().as_millis() as u64;
        let lost_packets = sent_packets - received_packets;
        let loss_percentage = if sent_packets > 0 {
            (lost_packets as f64 / sent_packets as f64) * 100.0
        } else {
            0.0
        };
        
        let grade = Self::calculate_grade(loss_percentage);
        
        Ok(PacketLossResult {
            sent_packets,
            received_packets,
            lost_packets,
            loss_percentage,
            grade,
            test_duration_ms: elapsed,
        })
    }

    fn calculate_grade(loss_percentage: f64) -> PacketLossGrade {
        match loss_percentage {
            x if x < 0.1 => PacketLossGrade::Excellent,
            x if x < 1.0 => PacketLossGrade::Good,
            x if x < 3.0 => PacketLossGrade::Fair,
            _ => PacketLossGrade::Poor,
        }
    }
}

impl PacketLossResult {
    /// Generate user-friendly summary
    pub fn summary(&self) -> String {
        let grade_emoji = match self.grade {
            PacketLossGrade::Excellent => "⭐",
            PacketLossGrade::Good => "✓",
            PacketLossGrade::Fair => "⚠️",
            PacketLossGrade::Poor => "🔴",
        };
        
        format!(
            "📊 Packet Loss Test Results\n\
             ━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
             Sent:     {} packets\n\
             Received: {} packets\n\
             Lost:     {} packets\n\
             Loss:     {:.2}% {}\n\
             Grade:    {:?}",
            self.sent_packets,
            self.received_packets,
            self.lost_packets,
            self.loss_percentage,
            grade_emoji,
            self.grade
        )
    }

    /// Get recommendation based on packet loss
    pub fn recommendation(&self) -> &str {
        match self.grade {
            PacketLossGrade::Excellent => {
                "Excellent! Your network has minimal packet loss. Perfect for VoIP, gaming, and streaming."
            }
            PacketLossGrade::Good => {
                "Good packet loss levels. Suitable for most online activities including video calls."
            }
            PacketLossGrade::Fair => {
                "Moderate packet loss detected. May cause occasional issues with real-time applications. \
                 Check your WiFi signal strength or contact your ISP."
            }
            PacketLossGrade::Poor => {
                "High packet loss detected! This will cause significant issues with video calls, gaming, \
                 and streaming. Recommendations:\n\
                 1. Check physical network cables\n\
                 2. Move closer to WiFi router\n\
                 3. Reduce interference from other devices\n\
                 4. Contact your ISP if problem persists"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grade_calculation() {
        assert!(matches!(PacketLossDetector::calculate_grade(0.05), PacketLossGrade::Excellent));
        assert!(matches!(PacketLossDetector::calculate_grade(0.5), PacketLossGrade::Good));
        assert!(matches!(PacketLossDetector::calculate_grade(2.0), PacketLossGrade::Fair));
        assert!(matches!(PacketLossDetector::calculate_grade(5.0), PacketLossGrade::Poor));
    }
}
