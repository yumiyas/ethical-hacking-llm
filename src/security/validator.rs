//! Input Validation
//! Validate and sanitize user inputs

use super::SecurityError;
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::warn;

static BLOCKED_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)rm\s+-rf\s+/\s*").unwrap(),
        Regex::new(r"(?i)format\s+c:\s*/q").unwrap(),
        Regex::new(r"(?i)drop\s+table").unwrap(),
        Regex::new(r"(?i)DELETE\s+FROM.*WHERE").unwrap(),
        Regex::new(r"(?i)shutdown\s+-s").unwrap(),
        Regex::new(r"(?i)rd\s+/s\s+/q").unwrap(),
        Regex::new(r"(?i)del\s+/f\s+/s").unwrap(),
        Regex::new(r"(?i):(){ :\|:& };:").unwrap(), // Fork bomb
        Regex::new(r"(?i)wget\s+.*\|\s*bash").unwrap(),
        Regex::new(r"(?i)curl\s+.*\|\s*bash").unwrap(),
Regex::new(r"(?i)chmod\s+777\s+/").unwrap(),
Regex::new(r"(?i)mkfs").unwrap(),
Regex::new(r"(?i)dd\s+if=.*of=/dev/").unwrap(),
]
});

static ALLOWED_CHARS: Lazy<Regex> = Lazy::new(|| {
Regex::new(r"^[a-zA-Z0-9\s-_.,!?@#$%^&*()+=<>?/\{}|~`"]+$").unwrap()
});

pub struct InputValidator {
max_length: usize,
blocked_commands: Vec<String>,
}

impl InputValidator {
pub fn new() -> Self {
Self {
max_length: 1000,
blocked_commands: vec![
"rm -rf".to_string(),
"format".to_string(),
"del /f".to_string(),
"shutdown".to_string(),
"dd if=".to_string(),
":(){".to_string(),
"chmod 777".to_string(),
"wget | bash".to_string(),
"curl | bash".to_string(),
],
}
}

pub fn validate(&self, input: &str) -> Result<(), SecurityError> {
// Check length
if input.len() > self.max_length {
return Err(SecurityError::TooLong { max: self.max_length });
}

// Check for empty input
if input.trim().is_empty() {
return Err(SecurityError::InvalidChars);
}

// Check allowed characters
if !ALLOWED_CHARS.is_match(input) {
warn!("Invalid characters detected in input: {}", input);
return Err(SecurityError::InvalidChars);
}

// Check blocked patterns
for pattern in BLOCKED_PATTERNS.iter() {
if pattern.is_match(input) {
warn!("Blocked pattern detected: {}", input);
return Err(SecurityError::BlockedPattern);
}
}

// Check blocked commands (case insensitive)
let input_lower = input.to_lowercase();
for cmd in &self.blocked_commands {
if input_lower.contains(&cmd.to_lowercase()) {
warn!("Blocked command detected: {}", cmd);
return Err(SecurityError::BlockedPattern);
}
}

Ok(())
}

pub fn sanitize(&self, input: &str) -> String {
input
.replace("<", "<")
.replace(">", ">")
.replace(""", """)
.replace("'", "'")
.replace("/", "/")
.replace("`", "`")
.replace("=", "=")
.replace("--", "--")
.replace(";", ";")
.replace("|", "|")
.replace("&", "&")
}

pub fn extract_safe_command(&self, input: &str) -> Option<String> {
// Extract only safe parts of the input
let words: Vec<&str> = input.split_whitespace().collect();
let safe_words: Vec<&str> = words.into_iter()
.filter(|w| !self.blocked_commands.iter().any(|cmd| w.contains(cmd)))
.collect();

if safe_words.is_empty() {
None
} else {
Some(safe_words.join(" "))
}
}
}

impl Default for InputValidator {
fn default() -> Self {
Self::new()
}
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_valid_input() {
let validator = InputValidator::new();
assert!(validator.validate("nmap -sV target.com").is_ok());
}

#[test]
fn test_blocked_pattern() {
let validator = InputValidator::new();
assert!(validator.validate("rm -rf /").is_err());
}

#[test]
fn test_sanitize() {
let validator = InputValidator::new();
let sanitized = validator.sanitize("<script>alert('xss')</script>");
assert_eq!(sanitized, "<script>alert('xss')</script>");
}
}
