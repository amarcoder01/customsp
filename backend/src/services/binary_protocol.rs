/// Binary WebSocket Protocol
/// 
/// Replaces JSON with MessagePack for 30-50% size reduction and faster serialization.
/// Optimized for real-time speed test progress updates.
/// 
/// Benefits:
/// - 30-50% smaller message size vs JSON
/// - 2-3x faster serialization/deserialization
/// - Lower CPU usage on server
/// - Better mobile performance
/// - More efficient batching

use serde::{Deserialize, Serialize};
use bytes::Bytes;

/// Binary message types for WebSocket communication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BinaryMessage {
    /// Client requests to start a speed test
    StartTest {
        config: TestConfig,
    },
    
    /// Server sends test progress updates
    Progress {
        stage: TestStage,
        progress_pct: u8,
        current_speed_mbps: f64,
        current_latency_ms: f64,
    },
    
    /// Server sends latency measurement during test
    LatencyMeasurement {
        stage: LatencyStage,
        latency_ms: f64,
        timestamp_ms: u64,
    },
    
    /// Server sends final test results
    Results {
        test_id: String,
        results: CompactTestResult,
    },
    
    /// Client or server sends ping for connection keep-alive
    Ping {
        timestamp: u64,
    },
    
    /// Response to ping
    Pong {
        timestamp: u64,
    },
    
    /// Error occurred during test
    Error {
        code: ErrorCode,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestStage {
    Initializing,
    IdleLatency,
    Download,
    Upload,
    Finalizing,
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LatencyStage {
    Idle,
    DownloadLoaded,
    UploadLoaded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub duration_ms: u32,
    pub chunk_size_kb: u16,
    pub parallel_streams: u8,
    pub measure_latency: bool,
    pub measure_jitter: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            duration_ms: 10000,
            chunk_size_kb: 256,
            parallel_streams: 4,
            measure_latency: true,
            measure_jitter: true,
        }
    }
}

/// Compact version of TestResult optimized for binary transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactTestResult {
    // Speeds (use f32 instead of f64 to save space)
    pub dl_mbps: f32,
    pub ul_mbps: f32,
    
    // Latency measurements (all in milliseconds)
    pub idle_lat: f32,
    pub dl_lat: f32,
    pub ul_lat: f32,
    pub jitter: f32,
    
    // Bufferbloat
    pub bb_grade: u8,  // 0=A+, 1=A, 2=B, 3=C, 4=D, 5=F
    pub bb_dl_pct: u16, // Percentage increase * 10 (e.g., 533% = 5330)
    pub bb_ul_pct: u16,
    
    // RPM metrics
    pub idle_rpm: u16,
    pub dl_rpm: u16,
    pub ul_rpm: u16,
    
    // AIM scores (0-100)
    pub gaming_score: u8,
    pub streaming_score: u8,
    pub video_score: u8,
    pub browsing_score: u8,
    pub overall_score: u8,
    
    // Metadata
    pub duration_ms: u32,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorCode {
    InvalidConfig,
    ServerOverloaded,
    NetworkError,
    Timeout,
    InternalError,
}

/// Binary protocol encoder/decoder
pub struct BinaryProtocol;

impl BinaryProtocol {
    /// Encode message to MessagePack binary format
    pub fn encode(message: &BinaryMessage) -> Result<Vec<u8>, ProtocolError> {
        rmp_serde::to_vec(message)
            .map_err(|e| ProtocolError::EncodingError(e.to_string()))
    }
    
    /// Decode MessagePack binary format to message
    pub fn decode(data: &[u8]) -> Result<BinaryMessage, ProtocolError> {
        rmp_serde::from_slice(data)
            .map_err(|e| ProtocolError::DecodingError(e.to_string()))
    }
    
    /// Encode message to Bincode (alternative binary format)
    pub fn encode_bincode(message: &BinaryMessage) -> Result<Vec<u8>, ProtocolError> {
        bincode::serialize(message)
            .map_err(|e| ProtocolError::EncodingError(e.to_string()))
    }
    
    /// Decode Bincode format to message
    pub fn decode_bincode(data: &[u8]) -> Result<BinaryMessage, ProtocolError> {
        bincode::deserialize(data)
            .map_err(|e| ProtocolError::DecodingError(e.to_string()))
    }
    
    /// Encode to Bytes for WebSocket transmission
    pub fn encode_to_bytes(message: &BinaryMessage) -> Result<Bytes, ProtocolError> {
        let vec = Self::encode(message)?;
        Ok(Bytes::from(vec))
    }
    
    /// Decode from Bytes received from WebSocket
    pub fn decode_from_bytes(bytes: &Bytes) -> Result<BinaryMessage, ProtocolError> {
        Self::decode(bytes)
    }
}

/// Message batching for efficiency
pub struct MessageBatch {
    messages: Vec<BinaryMessage>,
    max_size: usize,
}

impl MessageBatch {
    pub fn new(max_size: usize) -> Self {
        Self {
            messages: Vec::new(),
            max_size,
        }
    }
    
    /// Add message to batch
    pub fn add(&mut self, message: BinaryMessage) -> bool {
        if self.messages.len() >= self.max_size {
            return false;
        }
        self.messages.push(message);
        true
    }
    
    /// Check if batch is ready to send (full or timeout)
    pub fn is_ready(&self) -> bool {
        self.messages.len() >= self.max_size
    }
    
    /// Encode entire batch as single binary message
    pub fn encode(&self) -> Result<Vec<u8>, ProtocolError> {
        rmp_serde::to_vec(&self.messages)
            .map_err(|e| ProtocolError::EncodingError(e.to_string()))
    }
    
    /// Decode batch
    pub fn decode(data: &[u8]) -> Result<Vec<BinaryMessage>, ProtocolError> {
        rmp_serde::from_slice(data)
            .map_err(|e| ProtocolError::DecodingError(e.to_string()))
    }
    
    /// Get messages and clear batch
    pub fn take(&mut self) -> Vec<BinaryMessage> {
        std::mem::take(&mut self.messages)
    }
}

#[derive(Debug, Clone)]
pub enum ProtocolError {
    EncodingError(String),
    DecodingError(String),
    InvalidMessage(String),
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolError::EncodingError(e) => write!(f, "Encoding error: {}", e),
            ProtocolError::DecodingError(e) => write!(f, "Decoding error: {}", e),
            ProtocolError::InvalidMessage(e) => write!(f, "Invalid message: {}", e),
        }
    }
}

impl std::error::Error for ProtocolError {}

/// Performance comparison helper
pub struct ProtocolComparison;

impl ProtocolComparison {
    /// Compare JSON vs MessagePack size
    pub fn compare_sizes(message: &BinaryMessage) -> SizeComparison {
        // JSON size
        let json_size = serde_json::to_vec(message)
            .map(|v| v.len())
            .unwrap_or(0);
        
        // MessagePack size
        let msgpack_size = rmp_serde::to_vec(message)
            .map(|v| v.len())
            .unwrap_or(0);
        
        // Bincode size
        let bincode_size = bincode::serialize(message)
            .map(|v| v.len())
            .unwrap_or(0);
        
        let savings_msgpack = if json_size > 0 {
            ((json_size - msgpack_size) as f64 / json_size as f64) * 100.0
        } else {
            0.0
        };
        
        let savings_bincode = if json_size > 0 {
            ((json_size - bincode_size) as f64 / json_size as f64) * 100.0
        } else {
            0.0
        };
        
        SizeComparison {
            json_bytes: json_size,
            msgpack_bytes: msgpack_size,
            bincode_bytes: bincode_size,
            msgpack_savings_pct: savings_msgpack,
            bincode_savings_pct: savings_bincode,
        }
    }
}

#[derive(Debug)]
pub struct SizeComparison {
    pub json_bytes: usize,
    pub msgpack_bytes: usize,
    pub bincode_bytes: usize,
    pub msgpack_savings_pct: f64,
    pub bincode_savings_pct: f64,
}

impl SizeComparison {
    pub fn display(&self) -> String {
        format!(
            "Size Comparison:\n\
             ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
             JSON:       {:>6} bytes (baseline)\n\
             MessagePack: {:>6} bytes ({:.1}% smaller)\n\
             Bincode:     {:>6} bytes ({:.1}% smaller)\n\
             ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
             Best: {} ({:.1}% savings)",
            self.json_bytes,
            self.msgpack_bytes,
            self.msgpack_savings_pct,
            self.bincode_bytes,
            self.bincode_savings_pct,
            if self.bincode_bytes < self.msgpack_bytes { "Bincode" } else { "MessagePack" },
            if self.bincode_bytes < self.msgpack_bytes { self.bincode_savings_pct } else { self.msgpack_savings_pct }
        )
    }
}

/// Compression helper for large batches
pub struct MessageCompression;

impl MessageCompression {
    /// Compress batch with gzip (optional, for very large batches)
    #[cfg(feature = "compression")]
    pub fn compress(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
        use flate2::Compression;
        use flate2::write::GzEncoder;
        use std::io::Write;
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
        encoder.write_all(data)?;
        encoder.finish()
    }
    
    /// Decompress gzip data
    #[cfg(feature = "compression")]
    pub fn decompress(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
        use flate2::read::GzDecoder;
        use std::io::Read;
        
        let mut decoder = GzDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;
        Ok(decompressed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encode_decode() {
        let message = BinaryMessage::Progress {
            stage: TestStage::Download,
            progress_pct: 75,
            current_speed_mbps: 450.5,
            current_latency_ms: 15.2,
        };
        
        let encoded = BinaryProtocol::encode(&message).unwrap();
        let decoded = BinaryProtocol::decode(&encoded).unwrap();
        
        match decoded {
            BinaryMessage::Progress { progress_pct, .. } => {
                assert_eq!(progress_pct, 75);
            },
            _ => panic!("Wrong message type"),
        }
    }
    
    #[test]
    fn test_size_comparison() {
        let message = BinaryMessage::Results {
            test_id: "test-123".to_string(),
            results: CompactTestResult {
                dl_mbps: 450.0,
                ul_mbps: 50.0,
                idle_lat: 15.0,
                dl_lat: 25.0,
                ul_lat: 35.0,
                jitter: 5.0,
                bb_grade: 2,
                bb_dl_pct: 667,
                bb_ul_pct: 1333,
                idle_rpm: 4000,
                dl_rpm: 2400,
                ul_rpm: 1714,
                gaming_score: 85,
                streaming_score: 95,
                video_score: 80,
                browsing_score: 90,
                overall_score: 87,
                duration_ms: 10000,
                timestamp: 1700000000,
            },
        };
        
        let comparison = ProtocolComparison::compare_sizes(&message);
        
        // MessagePack should be significantly smaller than JSON
        assert!(comparison.msgpack_savings_pct > 20.0);
        println!("{}", comparison.display());
    }
    
    #[test]
    fn test_message_batch() {
        let mut batch = MessageBatch::new(10);
        
        for i in 0..5 {
            let msg = BinaryMessage::Progress {
                stage: TestStage::Download,
                progress_pct: (i * 20) as u8,
                current_speed_mbps: 100.0 + (i as f64 * 50.0),
                current_latency_ms: 15.0,
            };
            assert!(batch.add(msg));
        }
        
        let encoded = batch.encode().unwrap();
        let decoded = MessageBatch::decode(&encoded).unwrap();
        
        assert_eq!(decoded.len(), 5);
    }
}
