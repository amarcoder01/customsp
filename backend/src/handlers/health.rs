use actix_web::{web, HttpResponse, Result};
use log::info;
use std::time::SystemTime;

use crate::models::HealthStatus;

static START_TIME: once_cell::sync::Lazy<SystemTime> = once_cell::sync::Lazy::new(SystemTime::now);

pub async fn health_check() -> Result<HttpResponse> {
    info!("Health check requested");
    
    let uptime = START_TIME
        .elapsed()
        .map(|d| d.as_secs())
        .unwrap_or(0);
    
    let status = HealthStatus {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: uptime,
        active_tests: 0, // TODO: Implement test counter
    };
    
    Ok(HttpResponse::Ok().json(status))
}
