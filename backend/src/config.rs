use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server_id: String,
    pub server_name: String,
    pub server_ip: String,
    pub server_lat: f64,
    pub server_lon: f64,
    pub bind_host: String,
    pub bind_port: u16,
    pub max_concurrent_tests: usize,
    pub database_path: String,
    pub default_test_duration_ms: u64,
    pub chunk_size_bytes: usize,
    pub min_test_duration_ms: u64,
    pub max_test_duration_ms: u64,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            server_id: env::var("SERVER_ID").unwrap_or_else(|_| "mumbai-01".to_string()),
            server_name: env::var("SERVER_NAME").unwrap_or_else(|_| "Mumbai, India".to_string()),
            server_ip: env::var("SERVER_IP").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_lat: env::var("SERVER_LAT")
                .unwrap_or_else(|_| "19.0760".to_string())
                .parse()
                .unwrap_or(19.0760),
            server_lon: env::var("SERVER_LON")
                .unwrap_or_else(|_| "72.8777".to_string())
                .parse()
                .unwrap_or(72.8777),
            bind_host: env::var("BIND_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            bind_port: env::var("BIND_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            max_concurrent_tests: env::var("MAX_CONCURRENT_TESTS")
                .unwrap_or_else(|_| "50".to_string())
                .parse()
                .unwrap_or(50),
            database_path: env::var("DATABASE_PATH")
                .unwrap_or_else(|_| "./data/speedtest.db".to_string()),
            default_test_duration_ms: env::var("DEFAULT_TEST_DURATION_MS")
                .unwrap_or_else(|_| "10000".to_string())
                .parse()
                .unwrap_or(10000),
            chunk_size_bytes: env::var("CHUNK_SIZE_BYTES")
                .unwrap_or_else(|_| "65536".to_string())
                .parse()
                .unwrap_or(65536),
            min_test_duration_ms: env::var("MIN_TEST_DURATION_MS")
                .unwrap_or_else(|_| "5000".to_string())
                .parse()
                .unwrap_or(5000),
            max_test_duration_ms: env::var("MAX_TEST_DURATION_MS")
                .unwrap_or_else(|_| "30000".to_string())
                .parse()
                .unwrap_or(30000),
        }
    }
}
