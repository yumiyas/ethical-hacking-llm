//! API-based Model Clients
//! Connects to external LLM APIs (Ollama, HuggingFace, Groq)

use super::{ModelTrait, ModelType};
use anyhow::{Result, Context};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;
use tracing::{info, debug, warn};

#[derive(Debug)]
pub struct OllamaClient {
    client: Client,
    endpoint: String,
    model: String,
    temperature: f32,
}

impl OllamaClient {
    pub fn new(endpoint: &str, model: &str, temperature: f32) -> Result<Self> {
        info!("Creating Ollama client for model: {} at {}", model, endpoint);
        
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()?;

        Ok(Self {
            client,
            endpoint: endpoint.trim_end_matches('/').to_string(),
            model: model.to_string(),
            temperature,
        })
    }

    async fn check_health(&self) -> Result<bool> {
        let response = self.client
            .get(&format!("{}/api/tags", self.endpoint))
            .send()
            .await?;
        
        Ok(response.status().is_success())
    }
}

#[async_trait]
impl ModelTrait for OllamaClient {
    async fn generate(&self, prompt: &str, max_tokens: usize) -> Result<String> {
        debug!("Calling Ollama API with prompt: {}", prompt);

        let response = self.client
            .post(&format!("{}/api/generate", self.endpoint))
            .json(&json!({
                "model": self.model,
                "prompt": prompt,
                "stream": false,
                "options": {
                    "num_predict": max_tokens,
                    "temperature": self.temperature,
                }
            }))
            .send()
            .await
            .context("Ollama API request failed")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            anyhow::bail!("Ollama API returned {}: {}", status, text);
        }

        let data: Value = response.json().await?;
        let response_text = data["response"]
            .as_str()
            .unwrap_or("")
            .to_string();

        debug!("Received response of length: {}", response_text.len());
        Ok(response_text)
    }

    fn name(&self) -> &'static str {
        "Ollama"
    }

    fn model_type(&self) -> ModelType {
        ModelType::Ollama
    }

    async fn is_available(&self) -> bool {
        self.check_health().await.unwrap_or(false)
    }
}

#[derive(Debug)]
pub struct HuggingFaceClient {
    client: Client,
    api_token: String,
    model: String,
    temperature: f32,
}

impl HuggingFaceClient {
    pub fn new(api_token: &str, model: &str, temperature: f32) -> Self {
        info!("Creating HuggingFace client for model: {}", model);
        
        Self {
            client: Client::new(),
            api_token: api_token.to_string(),
            model: model.to_string(),
            temperature,
        }
    }
}

#[async_trait]
impl ModelTrait for HuggingFaceClient {
    async fn generate(&self, prompt: &str, max_tokens: usize) -> Result<String> {
        debug!("Calling HuggingFace API");

        let response = self.client
            .post(&format!(
                "https://api-inference.huggingface.co/models/{}",
                self.model
            ))
            .header("Authorization", format!("Bearer {}", self.api_token))
            .json(&json!({
                "inputs": prompt,
                "parameters": {
                    "max_new_tokens": max_tokens,
                    "temperature": self.temperature,
                    "return_full_text": false
                }
            }))
            .send()
            .await
            .context("HuggingFace API request failed")?;

        if !response.status().is_success() {
            let status = response.status();
            if status == 503 {
                // Model is loading
                warn!("HuggingFace model is loading, waiting...");
                tokio::time::sleep(Duration::from_secs(5)).await;
                return self.generate(prompt, max_tokens).await;
            }
            anyhow::bail!("HuggingFace API returned {}", status);
        }

        let data: Value = response.json().await?;
        let response_text = if data.is_array() {
            data[0]["generated_text"].as_str().unwrap_or("").to_string()
        } else {
            data["generated_text"].as_str().unwrap_or("").to_string()
        };

        Ok(response_text)
    }

    fn name(&self) -> &'static str {
        "HuggingFace"
    }

    fn model_type(&self) -> ModelType {
        ModelType::Api
    }
}

#[derive(Debug)]
pub struct GroqClient {
    client: Client,
    api_key: String,
    model: String,
    temperature: f32,
}

impl GroqClient {
    pub fn new(api_key: &str, model: &str, temperature: f32) -> Self {
        info!("Creating Groq client for model: {}", model);
        
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            model: model.to_string(),
            temperature,
        }
    }
}

#[async_trait]
impl ModelTrait for GroqClient {
    async fn generate(&self, prompt: &str, max_tokens: usize) -> Result<String> {
        debug!("Calling Groq API");

        let response = self.client
            .post("https://api.groq.com/openai/v1/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "model": self.model,
                "prompt": prompt,
                "max_tokens": max_tokens,
                "temperature": self.temperature,
            }))
            .send()
            .await
            .context("Groq API request failed")?;

        if !response.status().is_success() {
            anyhow::bail!("Groq API returned {}", response.status());
        }

        let data: Value = response.json().await?;
        let response_text = data["choices"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(response_text)
    }

    fn name(&self) -> &'static str {
        "Groq"
    }

    fn model_type(&self) -> ModelType {
        ModelType::Api
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ollama_client_creation() {
        let client = OllamaClient::new("http://localhost:11434", "llama2", 0.7);
        assert!(client.is_ok());
    }
}
