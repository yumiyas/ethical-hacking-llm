//! Quantized Model Implementation
//! 4-bit and 8-bit quantized models for faster inference

use super::{ModelTrait, ModelType};
use anyhow::{Result, Context};
use async_trait::async_trait;
use candle_core::{Device, Tensor};
use candle_transformers::models::quantized_llama as model;
use tokenizers::Tokenizer;
use tracing::{info, debug};

#[derive(Debug)]
pub struct QuantizedModel {
    model: model::Model,
    tokenizer: Tokenizer,
    device: Device,
    bits: u8,
    model_name: String,
    temperature: f32,
}

impl QuantizedModel {
    pub fn new(
        model_path: &str,
        tokenizer_path: &str,
        bits: u8,
        temperature: f32,
    ) -> Result<Self> {
        info!("Loading quantized {}

-bit model from: {}", model_path);

let device = Device::Cpu;

// Load quantized model
let model = model::Model::from_gguf(model_path, &device)
.with_context(|| format!("Failed to load quantized model from {}", model_path))?;

// Load tokenizer
let tokenizer = Tokenizer::from_file(tokenizer_path)
.with_context(|| format!("Failed to load tokenizer from {}", tokenizer_path))?;

info!("✅ Quantized model loaded ({} bits)", bits);

Ok(Self {
model,
tokenizer,
device,
bits,
model_name: format!("phi-2-{}bit", bits),
temperature,
})
}

fn prepare_prompt(&self, prompt: &str) -> String {
format!(
"<s>[INST] <<SYS>>\nYou are an ethical hacking assistant. Provide safe and educational information about cybersecurity.\n<</SYS>>\n\n{} [/INST]",
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
impl ModelTrait for QuantizedModel {
async fn generate(&self, prompt: &str, max_tokens: usize) -> Result<String> {
debug!("Generating with quantized model, prompt: {}", prompt);

let processed_prompt = self.prepare_prompt(prompt);

// Tokenize input
let mut tokens = self.tokenizer
.encode(processed_prompt, true)
.context("Tokenization failed")?
.get_ids()
.to_vec();

// Generate tokens
for i in 0..max_tokens {
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

// Safety limit
if tokens.len() > 2048 {
debug!("Reached max tokens limit");
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
"QuantizedModel"
}

fn model_type(&self) -> ModelType {
ModelType::Quantized
}

async fn is_available(&self) -> bool {
true
}
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_prepare_prompt() {
let model = QuantizedModel::new("dummy", "dummy", 4, 0.7).unwrap();
let prompt = "Test question";
let prepared = model.prepare_prompt(prompt);
assert!(prepared.contains(prompt));
assert!(prepared.contains("ethical hacking"));
}
}

