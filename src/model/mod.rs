//! Model Module
//! Handles different types of LLM models (local, quantized, API-based)

pub mod local_model;
pub mod api_clients;
pub mod quantized;

use anyhow::Result;
use async_trait::async_trait;
use std::fmt::Debug;

/// Trait for all model implementations
#[async_trait]
pub trait ModelTrait: Send + Sync + Debug {
    /// Generate response from prompt
    async fn generate(&self, prompt: &str, max_tokens: usize) -> Result<String>;
    
    /// Get model name
    fn name(&self) -> &'static str;
    
    /// Get model type
    fn model_type(&self) -> ModelType;
    
    /// Check if model is available
    async fn is_available(&self) -> bool {
        true
    }
}

/// Types of models supported
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelType {
    Local,
    Quantized,
    Api,
    Ollama,
}

impl std::fmt::Display for ModelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelType::Local => write!(f, "local"),
            ModelType::Quantized => write!(f, "quantized"),
            ModelType::Api => write!(f, "api"),
            ModelType::Ollama => write!(f, "ollama"),
        }
    }
}

/// Model configuration
#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub model_type: ModelType,
    pub path: String,
    pub tokenizer_path: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub top_p: f32,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            model_type: ModelType::Quantized,
            path: "models/phi-2-q4/phi-2-q4.gguf".to_string(),
            tokenizer_path: "models/phi-2-q4/tokenizer.json".to_string(),
            max_tokens: 512,
            temperature: 0.7,
            top_p: 0.9,
        }
    }
}

/// Model router for managing multiple models
pub mod router {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[derive(Debug)]
    pub struct ModelRouter {
        models: Arc<RwLock<HashMap<String, Box<dyn ModelTrait>>>>,
        default_model: String,
    }

    impl ModelRouter {
        pub fn new(default_model: String) -> Self {
            Self {
                models: Arc::new(RwLock::new(HashMap::new())),
                default_model,
            }
        }

        pub async fn register_model(&self, name: &str, model: Box<dyn ModelTrait>) {
            let mut models = self.models.write().await;
            models.insert(name.to_string(), model);
        }

        pub async fn get_model(&self, name: Option<&str>) -> Option<tokio::sync::RwLockReadGuard<'_, Box<dyn ModelTrait>>> {
            let models = self.models.read().await;
            let name = name.unwrap_or(&self.default_model);
            
            if models.contains_key(name) {
                // This is a bit tricky with RwLock, but for simplicity we'll return None
                // In real implementation, you'd want to return a reference
                None
            } else {
                None
            }
        }

        pub async fn list_models(&self) -> Vec<String> {
            let models = self.models.read().await;
            models.keys().cloned().collect()
        }
    }
}
