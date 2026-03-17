//! Metrics Collection
//! Prometheus metrics for monitoring

use lazy_static::lazy_static;
use prometheus::{
    register_counter_vec, register_gauge, register_histogram_vec,
    register_int_counter, register_int_counter_vec, register_int_gauge,
    CounterVec, Gauge, HistogramVec, IntCounter, IntCounterVec, IntGauge,
    opts, register_int_gauge_vec, IntGaugeVec,
};
use std::sync::atomic::{AtomicU64, Ordering};

lazy_static! {
    static ref HTTP_REQUESTS_TOTAL: IntCounterVec = register_int_counter_vec!(
        opts!("http_requests_total", "Total HTTP requests"),
        &["method", "path", "status"]
    ).unwrap();

    static ref HTTP_REQUEST_DURATION: HistogramVec = register_histogram_vec!(
        opts!("http_request_duration_ms", "HTTP request duration in milliseconds"),
        &["method", "path"]
    ).unwrap();

    static ref INFERENCE_REQUESTS_TOTAL: IntCounterVec = register_int_counter_vec!(
        opts!("inference_requests_total", "Total inference requests"),
        &["model"]
    ).unwrap();

    static ref INFERENCE_DURATION: HistogramVec = register_histogram_vec!(
        opts!("inference_duration_ms", "Inference duration in milliseconds"),
        &["model"]
    ).unwrap();

    static ref INFERENCE_TOKENS: IntCounterVec = register_int_counter_vec!(
        opts!("inference_tokens_total", "Total tokens generated"),
        &["model"]
    ).unwrap();

    static ref CACHE_HITS: IntCounter = register_int_counter!(
        opts!("cache_hits_total", "Total cache hits")
    ).unwrap();

    static ref CACHE_MISSES: IntCounter = register_int_counter!(
        opts!("cache_misses_total", "Total cache misses")
    ).unwrap();

    static ref ACTIVE_REQUESTS: IntGauge = register_int_gauge!(
        opts!("active_requests", "Number of active requests")
    ).unwrap();

    static ref MODEL_MEMORY_USAGE: IntGaugeVec = register_int_gauge_vec!(
        opts!("model_memory_usage_bytes", "Model memory usage in bytes"),
        &["model"]
    ).unwrap();

    static ref ERROR_COUNT: IntCounterVec = register_int_counter_vec!(
        opts!("error_count_total", "Total errors by type"),
        &["type"]
    ).unwrap();

    static ref QUERY_LENGTH: HistogramVec = register_histogram_vec!(
        opts!("query_length_bytes", "Query length in bytes"),
        &["type"]
    ).unwrap();
}

static TOTAL_QUERIES: AtomicU64 = AtomicU64::new(0);
static TOTAL_LATENCY: AtomicU64 = AtomicU64::new(0);
static CACHE_HIT_COUNT: AtomicU64 = AtomicU64::new(0);
static CACHE_MISS_COUNT: AtomicU64 = AtomicU64::new(0);

/// Initialize metrics system
pub fn init_metrics() {
    tracing::info!("📊 Metrics initialized");
}

/// Record HTTP request
pub fn record_http_request(method: &str, path: &str, status: u16, duration_ms: f64) {
    HTTP_REQUESTS_TOTAL
        .with_label_values(&[method, path, &status.to_string()])
        .inc();
    
    HTTP_REQUEST_DURATION
        .with_label_values(&[method, path])
        .observe(duration_ms);
}

/// Record inference request
pub fn record_inference(model: &str, tokens: usize, duration_ms: f64) {
    INFERENCE_REQUESTS_TOTAL.with_label_values(&[model]).inc();
    INFERENCE_DURATION.with_label_values(&[model]).observe(duration_ms);
    INFERENCE_TOKENS.with_label_values(&[model]).inc_by(tokens as u64);
}

/// Record cache hit
pub fn record_cache_hit() {
    CACHE_HITS.inc();
    CACHE_HIT_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Record cache miss
pub fn record_cache_miss() {
    CACHE_MISSES.inc();
    CACHE_MISS_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Record active request
pub fn record_request_start() {
    ACTIVE_REQUESTS.inc();
}

/// Record request end
pub fn record_request_end(duration_ms: f64, status: u16) {
    ACTIVE_REQUESTS.dec();
    
    if status >= 500 {
        ERROR_COUNT.with_label_values(&["server"]).inc();
    } else if status >= 400 {
        ERROR_COUNT.with_label_values(&["client"]).inc();
    }
    
    TOTAL_QUERIES.fetch_add(1, Ordering::Relaxed);
    TOTAL_LATENCY.fetch_add(duration_ms as u64, Ordering::Relaxed);
}

/// Record query
pub fn record_query(length: u64, duration_ms: u64) {
    TOTAL_QUERIES.fetch_add(1, Ordering::Relaxed);
    TOTAL_LATENCY.fetch_add(duration_ms, Ordering::Relaxed);
    QUERY_LENGTH.with_label_values(&["query"]).observe(length as f64);
}

/// Record error
pub fn record_error(error_type: &str) {
    ERROR_COUNT.with_label_values(&[error_type]).inc();
}

/// Set model memory usage
pub fn set_model_memory_usage(model: &str, bytes: u64) {
    MODEL_MEMORY_USAGE.with_label_values(&[model]).set(bytes as i64);
}

/// Get total queries
pub fn get_total_queries() -> u64 {
    TOTAL_QUERIES.load(Ordering::Relaxed)
}

/// Get average latency
pub fn get_avg_latency() -> f64 {
    let total = TOTAL_QUERIES.load(Ordering::Relaxed);
    if total == 0 {
        0.0
    } else {
        TOTAL_LATENCY.load(Ordering::Relaxed) as f64 / total as f64
    }
}

/// Get cache hits
pub fn get_cache_hits() -> u64 {
    CACHE_HIT_COUNT.load(Ordering::Relaxed)
}

/// Get cache misses
pub fn get_cache_misses() -> u64 {
    CACHE_MISS_COUNT.load(Ordering::Relaxed)
}

/// Get active requests
pub fn get_active_requests() -> i64 {
    ACTIVE_REQUESTS.get()
}

/// Gather all metrics
pub fn gather_metrics() -> String {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

/// Flush metrics (for shutdown)
pub fn flush_metrics() {
    tracing::info!("Flushing metrics...");
    // Additional flush logic if needed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_recording() {
        record_http_request("POST", "/query", 200, 45.6);
        record_inference("quantized", 100, 45.6);
        record_cache_hit();
        
        assert!(get_total_queries() > 0);
    }
}
