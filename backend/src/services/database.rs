use sqlx::{sqlite::{SqlitePool, SqliteConnectOptions}, Row};
use std::path::Path;
use log::{info, error};

use crate::models::TestResult;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Normalize database path for filesystem operations (strip sqlite prefixes)
        let fs_path = database_path
            .strip_prefix("sqlite://")
            .or_else(|| database_path.strip_prefix("sqlite:"))
            .unwrap_or(database_path);

        let is_memory = fs_path == ":memory:";

        let absolute_path = if is_memory {
            None
        } else {
            let path = Path::new(fs_path);
            let mut path_buf = if path.is_absolute() {
                path.to_path_buf()
            } else {
                std::env::current_dir()?.join(path)
            };

            // Create data directory if it doesn't exist
            if let Some(parent) = path_buf.parent() {
                std::fs::create_dir_all(parent)?;
            }

            Some(path_buf)
        };
        
        let pool = if is_memory {
            info!("Connecting to database: sqlite::memory:");
            SqlitePool::connect("sqlite::memory:").await?
        } else {
            let path = absolute_path.unwrap();
            info!("Connecting to database file: {}", path.display());
            let options = SqliteConnectOptions::new()
                .filename(&path)
                .create_if_missing(true);
            SqlitePool::connect_with(options).await?
        };
        
        // Create tables
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS test_results (
                id TEXT PRIMARY KEY,
                server_id TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                download_mbps REAL NOT NULL,
                upload_mbps REAL NOT NULL,
                latency_ms REAL NOT NULL,
                jitter_ms REAL NOT NULL,
                protocol TEXT NOT NULL,
                client_ip TEXT NOT NULL,
                test_duration_ms INTEGER NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;

        // Ensure legacy databases have the client_ip column
        let _ = sqlx::query(
            "ALTER TABLE test_results ADD COLUMN client_ip TEXT DEFAULT 'unknown'"
        )
        .execute(&pool)
        .await;
        
        // Create index for faster queries
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_timestamp ON test_results(timestamp DESC)"
        )
        .execute(&pool)
        .await?;
        
        info!("Database initialized successfully");
        
        Ok(Self { pool })
    }
    
    pub async fn save_test_result(&self, result: &TestResult) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query(
            r#"
            INSERT INTO test_results 
            (id, server_id, timestamp, download_mbps, upload_mbps, latency_ms, jitter_ms, protocol, client_ip, test_duration_ms)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&result.id)
        .bind(&result.server_id)
        .bind(result.timestamp.to_rfc3339())
        .bind(result.download_mbps)
        .bind(result.upload_mbps)
        .bind(result.latency_ms)
        .bind(result.jitter_ms)
        .bind(&result.protocol)
        .bind(&result.client_ip)
        .bind(result.test_duration_ms as i64)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn get_test_result(&self, test_id: &str) -> Result<Option<TestResult>, Box<dyn std::error::Error>> {
        let row = sqlx::query(
            "SELECT * FROM test_results WHERE id = ? LIMIT 1"
        )
        .bind(test_id)
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let timestamp_str: String = row.get("timestamp");
                let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)?
                    .with_timezone(&chrono::Utc);
                
                Ok(Some(TestResult {
                    id: row.get("id"),
                    server_id: row.get("server_id"),
                    timestamp,
                    download_mbps: row.get("download_mbps"),
                    upload_mbps: row.get("upload_mbps"),
                    latency_ms: row.get("latency_ms"),
                    jitter_ms: row.get("jitter_ms"),
                    protocol: row.get("protocol"),
                    client_ip: row.get("client_ip"),
                    test_duration_ms: row.get::<i64, _>("test_duration_ms") as u64,
                }))
            }
            None => Ok(None),
        }
    }
    
    pub async fn get_test_history(&self, limit: i32) -> Result<Vec<TestResult>, Box<dyn std::error::Error>> {
        let rows = sqlx::query(
            "SELECT * FROM test_results ORDER BY timestamp DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        
        let mut results = Vec::new();
        for row in rows {
            let timestamp_str: String = row.get("timestamp");
            let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)?
                .with_timezone(&chrono::Utc);
            
            results.push(TestResult {
                id: row.get("id"),
                server_id: row.get("server_id"),
                timestamp,
                download_mbps: row.get("download_mbps"),
                upload_mbps: row.get("upload_mbps"),
                latency_ms: row.get("latency_ms"),
                jitter_ms: row.get("jitter_ms"),
                protocol: row.get("protocol"),
                client_ip: row.get("client_ip"),
                test_duration_ms: row.get::<i64, _>("test_duration_ms") as u64,
            });
        }
        
        Ok(results)
    }
}
