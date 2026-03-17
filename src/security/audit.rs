//! Audit Logging
//! Log all security-relevant events

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub timestamp: DateTime<Utc>,
    pub event_type: AuditEventType,
    pub user_id: Option<String>,
    pub ip_address: String,
    pub user_agent: String,
    pub action: String,
    pub resource: String,
    pub result: String,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    Query,
    ModelLoad,
    ModelUnload,
    ConfigChange,
    AuthSuccess,
    AuthFailure,
    RateLimitExceeded,
    SecurityViolation,
    SystemStartup,
    SystemShutdown,
}

pub struct AuditLogger {
    log_file: String,
    buffer: Arc<Mutex<Vec<AuditLog>>>,
    max_buffer_size: usize,
}

impl AuditLogger {
    pub fn new(log_file: &str) -> std::io::Result<Self> {
        // Create log file if it doesn't exist
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)?;

        Ok(Self {
            log_file: log_file.to_string(),
            buffer: Arc::new(Mutex::new(Vec::new())),
            max_buffer_size: 100,
        })
    }

    pub async fn log(&self, event: AuditLog) -> anyhow::Result<()> {
        let mut buffer = self.buffer.lock().await;
        buffer.push(event);

        if buffer.len() >= self.max_buffer_size {
            self.flush().await?;
        }

        Ok(())
    }

    pub async fn flush(&self) -> anyhow::Result<()> {
        let mut buffer = self.buffer.lock().await;
        
        if buffer.is_empty() {
            return Ok(());
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)?;

        for event in buffer.iter() {
            let line = serde_json::to_string(event)?;
            writeln!(file, "{}", line)?;
        }

        buffer.clear();
        info!("Flushed {} audit logs", buffer.len());
        
        Ok(())
    }

    pub async fn log_query(
        &self,
        query: &str,
        ip: &str,
        user_agent: &str,
        response_length: usize,
        processing_time_ms: u64,
        success: bool,
    ) -> anyhow::Result<()> {
        let event = AuditLog {
            timestamp: Utc::now(),
            event_type: AuditEventType::Query,
            user_id: None,
            ip_address: ip.to_string(),
            user_agent: user_agent.to_string(),
            action: "query".to_string(),
            resource: "llm".to_string(),
            result: if success { "success" } else { "failure" }.to_string(),
            details: serde_json::json!({
                "query_length": query.len(),
                "response_length": response_length,
                "processing_time_ms": processing_time_ms,
                "query_preview": &query[..query.len().min(100)],
            }),
        };

        self.log(event).await
    }

    pub async fn log_security_violation(
        &self,
        violation_type: &str,
        ip: &str,
        details: serde_json::Value,
    ) -> anyhow::Result<()> {
        let event = AuditLog {
            timestamp: Utc::now(),
            event_type: AuditEventType::SecurityViolation,
            user_id: None,
            ip_address: ip.to_string(),
            user_agent: "".to_string(),
            action: violation_type.to_string(),
            resource: "system".to_string(),
            result: "blocked".to_string(),
            details,
        };

        self.log(event).await
    }

    pub async fn log_rate_limit(
        &self,
        ip: &str,
        user_agent: &str,
    ) -> anyhow::Result<()> {
        let event = AuditLog {
            timestamp: Utc::now(),
            event_type: AuditEventType::RateLimitExceeded,
            user_id: None,
            ip_address: ip.to_string(),
            user_agent: user_agent.to_string(),
            action: "rate_limit".to_string(),
            resource: "api".to_string(),
            result: "blocked".to_string(),
            details: serde_json::json!({}),
        };

        self.log(event).await
    }

    pub async fn log_system_event(
        &self,
        event_type: AuditEventType,
        action: &str,
        details: serde_json::Value,
    ) -> anyhow::Result<()> {
        let event = AuditLog {
            timestamp: Utc::now(),
            event_type,
            user_id: None,
            ip_address: "system".to_string(),
            user_agent: "system".to_string(),
            action: action.to_string(),
            resource: "system".to_string(),
            result: "success".to_string(),
            details,
        };

        self.log(event).await
    }
}

impl Drop for AuditLogger {
    fn drop(&mut self) {
        // Flush on drop
        let buffer = std::mem::take(&mut *self.buffer.try_lock().unwrap_or_default());
        if !buffer.is_empty() {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.log_file) 
            {
                for event in buffer {
                    let _ = writeln!(file, "{}", serde_json::to_string(&event).unwrap());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_audit_logging() {
        let temp_file = NamedTempFile::new().unwrap();
        let logger = AuditLogger::new(temp_file.path().to_str().unwrap()).unwrap();

        logger.log_query("test query", "127.0.0.1", "test-agent", 100, 50, true).await.unwrap();
        logger.flush().await.unwrap();

        // Verify file was written
        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert!(!content.is_empty());
    }
}
