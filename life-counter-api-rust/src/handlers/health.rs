use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use crate::db::DbPool;

// Global log buffer to store recent logs
lazy_static::lazy_static! {
    static ref LOG_BUFFER: Arc<Mutex<VecDeque<LogEntry>>> = Arc::new(Mutex::new(VecDeque::with_capacity(1000)));
}

#[derive(Clone, Serialize)]
pub struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

impl LogEntry {
    pub fn new(level: &str, message: String) -> Self {
        Self {
            timestamp: Utc::now().to_rfc3339(),
            level: level.to_string(),
            message,
        }
    }
}

/// Add a log entry to the buffer
pub fn add_log_entry(level: &str, message: String) {
    let entry = LogEntry::new(level, message);
    if let Ok(mut buffer) = LOG_BUFFER.lock() {
        if buffer.len() >= 1000 {
            buffer.pop_front();
        }
        buffer.push_back(entry);
    }
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    timestamp: String,
    version: String,
}

#[derive(Serialize)]
struct DetailedHealthResponse {
    status: String,
    timestamp: String,
    version: String,
    database: DatabaseHealth,
    uptime: String,
}

#[derive(Serialize)]
struct DatabaseHealth {
    status: String,
    connected: bool,
}

#[derive(Serialize)]
struct LogsResponse {
    total: usize,
    logs: Vec<LogEntry>,
}

/// GET /health - Basic health check
pub async fn health_check() -> impl Responder {
    let response = HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    HttpResponse::Ok().json(response)
}

/// GET /health/detailed - Detailed health check including database connectivity
pub async fn health_check_detailed(pool: web::Data<DbPool>) -> impl Responder {
    // Check database connectivity
    let db_health = match pool.get() {
        Ok(_) => DatabaseHealth {
            status: "healthy".to_string(),
            connected: true,
        },
        Err(e) => {
            log::error!("Database connection failed: {}", e);
            DatabaseHealth {
                status: "unhealthy".to_string(),
                connected: false,
            }
        }
    };

    let overall_status = if db_health.connected {
        "healthy"
    } else {
        "degraded"
    };

    let response = DetailedHealthResponse {
        status: overall_status.to_string(),
        timestamp: Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_health,
        uptime: "N/A".to_string(), // Can be enhanced with actual uptime tracking
    };

    if overall_status == "healthy" {
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::ServiceUnavailable().json(response)
    }
}

/// GET /health/logs - Get recent application logs
/// Query parameters:
/// - limit: Number of logs to return (default: 100, max: 1000)
pub async fn get_logs(query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let limit = query
        .get("limit")
        .and_then(|l| l.parse::<usize>().ok())
        .unwrap_or(100)
        .min(1000);

    let logs = if let Ok(buffer) = LOG_BUFFER.lock() {
        buffer.iter()
            .rev()
            .take(limit)
            .cloned()
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let response = LogsResponse {
        total: logs.len(),
        logs,
    };

    HttpResponse::Ok().json(response)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("", web::get().to(health_check))
            .route("/detailed", web::get().to(health_check_detailed))
            .route("/logs", web::get().to(get_logs)),
    );
}
