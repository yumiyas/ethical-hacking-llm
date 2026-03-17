//! Local Model Implementation
//! Runs models directly on CPU/GPU using Candle

use super::{ModelTrait, ModelType};
use anyhow::{Result, Context};
use async_trait::async_trait;
use candle_core::{Device, Tensor};
use candle_transformers::models::quantized_llama as model;
use tokenizers::Tokenizer;
use tracing::{info, debug, warn};

#[derive(Debug)]
pub struct LocalModel {
    model: model::Model,
    tokenizer: Tokenizer,
    device: Device,
    max_seq_len: usize,
    model_name: String,
    temperature: f32,
}

impl LocalModel {
    pub fn new(
        model_path: &str,
        tokenizer_path: &str,
        temperature: f32,
    ) -> Result<Self> {
        info!("Loading local model from: {}", model_path);
        
        let device = Device::Cpu;
        
        // Load quantized model
        let model = model::Model::from_gguf(model_path, &device)
            .with_context(|| format!("Failed to load model from {}", model_path))?;
        
        // Load tokenizer
        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .with_context(|| format!("Failed to load tokenizer from {}", tokenizer_path))?;
        
        info!("✅ Model loaded successfully");
        
        Ok(Self {
            model,
            tokenizer,
            device,
            max_seq_len: 512,
            model_name: "phi-2".to_string(),
            temperature,
        })
    }

    fn prepare_prompt(&self, prompt: &str) -> String {
        format!(
            "<s>[INST] You are an ethical hacking assistant. Provide safe and educational information.
            Question: {} [/INST]",
            prompt
        )
    }

    fn sample_token(&self, logits: &Tensor) -> Result<u32> {
        if self.temperature <= 0.0 {
            // Greedy sampling
            let next_token = logits.argmax(0)?.to_scalar::<u32>()?;
            Ok(next_token)
        } else {
            // Temperature sampling
            let logits = logits.to_dtype(candle_core::DType::F32)?;
            let logits = (&logits / self.temperature)?;
            let prs = candle_nn::ops::softmax(&logits, 0)?;
            let prs: Vec<f32> = prs.to_vec1()?;
            let distr = rand::distributions::WeightedIndex::new(prs)?;
            let next_token = rand::thread_rng().sample(distr) as u32;
            Ok(next_token)
        }
    }
}

#[async_trait]
impl ModelTrait for LocalModel {
    async fn generate(&self, prompt: &str, max_tokens: usize) -> Result<String> {
        debug!("Generating response for prompt: {}", prompt);
        
        let processed_prompt = self.prepare_prompt(prompt);
        
        // Tokenize input
        let mut tokens = self.tokenizer
            .encode(processed_prompt, true)
            .context("Tokenization failed")?
            .get_ids()
            .to_vec();
        
        // Generate tokens
        for i in 0..max_tokens {
            if tokens.len() >= self.max_seq_len {
                warn!("Reached max sequence length");
                break;
            }

            let input = Tensor::new(&[tokens.as_slice()], &self.device)?
                .unsqueeze(0)?;
            
            let logits = self.model.forward(&input, 0)?;
            let next_token_logits = logits.squeeze(0)?.squeeze(0)?;
            
            let next_token = self.sample_token(&next_token_logits)?;
            tokens.push(next_token);

            // Stop on end token
            if next_token == self.tokenizer.token_to_id("</s>").unwrap_or(2) {
                debug!("Generated {} tokens", i + 1);
                break;
            }
        }

        // Decode response
        let response = self.tokenizer
            .decode(&tokens, true)
            .context("Decoding failed")?;

        // Extract just the response part
        let response = response
            .split("[/INST]")
            .last()
            .unwrap_or("")
            .trim()
            .to_string();

        Ok(response)
    }

    fn name(&self) -> &'static str {
        "LocalModel"
    }

    fn model_type(&self) -> ModelType {
        ModelType::Local
    }

    async fn is_available(&self) -> bool {
        true // Local model is always available
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_prompt() {
        let model = LocalModel::new("dummy", "dummy", 0.7).unwrap();
        let prompt = "Test question";
        let prepared = model.prepare_prompt(prompt);
        assert!(prepared.contains(prompt));
        assert!(prepared.contains("ethical hacking assistant"));
    }
}
