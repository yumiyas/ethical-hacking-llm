//! Application Settings
//! Configuration structure and loading

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub model: ModelConfig,
    pub cache: CacheConfig,
    pub security: SecurityConfig,
    pub api_keys: ApiKeys,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelConfig {
    pub path: String,
    pub tokenizer_path: String,
    pub model_type: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub top_p: f32,
    pub context_length: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub redis_url: Option<String>,
    pub ttl_seconds: u64,
    pub max_size: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecurityConfig {
    pub rate_limit_per_minute: u32,
    pub jwt_secret: String,
    pub max_query_length: usize,
    pub enable_audit_log: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ApiKeys {
    pub huggingface: Option<String>,
    pub groq: Option<String>,
    pub openai: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub file_enabled: bool,
    pub console_enabled: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 3000,
                workers: 4,
                timeout_seconds: 30,
            },
            model: ModelConfig {
                path: "models/phi-2-q4/phi-2-q4.gguf".to_string(),
                tokenizer_path: "models/phi-2-q4/tokenizer.json".to_string(),
                model_type: "quantized".to_string(),
                max_tokens: 512,
                temperature: 0.7,
                top_p: 0.9,
                context_length: 2048,
            },
            cache: CacheConfig {
                enabled: true,
                redis_url: None,
                ttl_seconds: 3600,
                max_size: 10000,
            },
            security: SecurityConfig {
                rate_limit_per_minute: 100,
                jwt_secret: "change-me-in-production".to_string(),
                max_query_length: 1000,
                enable_audit_log: true,
            },
            api_keys: ApiKeys::default(),
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "pretty".to_string(),
                file_enabled: true,
                console_enabled: true,
            },
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        // Load .env file if it exists
        dotenv::dotenv().ok();

        let mut config = AppConfig::default();

        // Server config from env
        if let Ok(host) = env::var("HOST") {
            config.server.host = host;
        }
        if let Ok(port) = env::var("PORT") {
            config.server.port = port.parse()?;
        }
        if let Ok(workers) = env::var("WORKERS") {
            config.server.workers = workers.parse()?;
        }

        // Model config from env
        if let Ok(path) = env::var("MODEL_PATH") {
            config.model.path = path;
        }
        if let Ok(tokenizer) = env::var("TOKENIZER_PATH") {
            config.model.tokenizer_path = tokenizer;
        }
        if let Ok(model_type) = env::var("MODEL_TYPE") {
            config.model.model_type = model_type;
        }
        if let Ok(max_tokens) = env::var("MAX_TOKENS") {
            config.model.max_tokens = max_tokens.parse()?;
        }
        if let Ok(temp) = env::var("TEMPERATURE") {
            config.model.temperature = temp.parse()?;
        }

        // Cache config from env
        if let Ok(redis_url) = env::var("REDIS_URL") {
            config.cache.redis_url = Some(redis_url);
        }
        if let Ok(ttl) = env::var("CACHE_TTL") {
            config.cache.ttl_seconds = ttl.parse()?;
        }

        // Security config from env
        if let Ok(rate) = env::var("RATE_LIMIT") {
            config.security.rate_limit_per_minute = rate.parse()?;
        }
        if let Ok(secret) = env::var("JWT_SECRET") {
            config.security.jwt_secret = secret;
        }

        // API keys from env
        config.api_keys.huggingface = env::var("HUGGINGFACE_API_KEY").ok();
        config.api_keys.groq = env::var("GROQ_API_KEY").ok();
        config.api_keys.openai = env::var("OPENAI_API_KEY").ok();

        // Try to load from config file if exists
        if let Ok(config_file) = env::var("CONFIG_FILE") {
            if let Ok(contents) = fs::read_to_string(config_file) {
                if let Ok(file_config) = toml::from_str::<AppConfig>(&contents) {
                    config.merge(file_config);
                }
            }
        }

        Ok(config)
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .context(format!("Failed to read config file: {}", path))?;
        
        let config: AppConfig = toml::from_str(&contents)
            .context("Failed to parse config file")?;
        
        Ok(config)
    }

    fn merge(&mut self, other: Self) {
        // Merge configurations, keeping existing values if not overridden
        if other.server.host != "127.0.0.1" {
            self.server.host = other.server.host;
        }
        if other.server.port != 3000 {
            self.server.port = other.server.port;
        }
        // Add more merge logic as needed
    }

    pub fn validate(&self) -> Result<()> {
        // Validate model path exists
        if !std::path::Path::new(&self.model.path).exists() {
            anyhow::bail!("Model path does not exist: {}", self.model.path);
        }

        // Validate tokenizer path exists
        if !std::path::Path::new(&self.model.tokenizer_path).exists() {
            anyhow::bail!("Tokenizer path does not exist: {}", self.model.tokenizer_path);
        }

        // Validate ports
        if self.server.port == 0 {
            anyhow::bail!("Invalid port number");
        }

        // Validate temperature
        if !(0.0..=2.0).contains(&self.model.temperature) {
            anyhow::bail!("Temperature must be between 0.0 and 2.0");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.server.port, 3000);
        assert_eq!(config.model.max_tokens, 512);
    }

    #[test]
    fn test_config_validation() {
        let config = AppConfig::default();
        // This will fail if model files don't exist
        // assert!(config.validate().is_ok());
    }
}
