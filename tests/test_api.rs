use ethical_hacking_llm::api::handlers;
use axum::http::StatusCode;
use serde_json::json;

#[tokio::test]
async fn test_health_check() {
    let response = handlers::health_check().await;
    assert_eq!(response.0, StatusCode::OK);
}

#[tokio::test]
async fn test_query_validation() {
    let config = std::sync::Arc::new(ethical_hacking_llm::config::AppConfig::default());
    
    let req = axum::Json(handlers::QueryRequest {
        query: "".to_string(),
        max_tokens: Some(100),
        temperature: 0.7,
        stream: false,
        model: None,
    });
    
    let response = handlers::handle_query(axum::extract::State(config), req).await;
    assert_eq!(response.0, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_models_endpoint() {
    let response = handlers::list_models().await;
    assert!(!response.0.is_empty());
}
