//! Ethical Hacking Knowledge Base
//! Commands, techniques, and tools for ethical hacking

pub mod commands;
pub mod techniques;
pub mod tools;

use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub syntax: String,
    pub examples: Vec<String>,
    pub category: CommandCategory,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommandCategory {
    Reconnaissance,
    Scanning,
    Enumeration,
    Exploitation,
    PostExploitation,
    Reporting,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Technique {
    pub name: String,
    pub description: String,
    pub mitre_id: Option<String>,
    pub steps: Vec<String>,
    pub tools: Vec<String>,
    pub mitigation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub installation: String,
    pub basic_usage: String,
    pub examples: Vec<String>,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CVE {
    pub id: String,
    pub description: String,
    pub cvss_score: f32,
    pub affected_software: Vec<String>,
    pub exploit_available: bool,
}

static KNOWLEDGE_BASE: Lazy<Arc<RwLock<KnowledgeBase>>> = Lazy::new(|| {
    Arc::new(RwLock::new(KnowledgeBase::new()))
});

pub struct KnowledgeBase {
    commands: HashMap<String, Command>,
    techniques: HashMap<String, Technique>,
    tools: HashMap<String, Tool>,
    cves: HashMap<String, CVE>,
}

impl KnowledgeBase {
    fn new() -> Self {
        Self {
            commands: HashMap::new(),
            techniques: HashMap::new(),
            tools: HashMap::new(),
            cves: HashMap::new(),
        }
    }

    pub async fn add_command(&mut self, cmd: Command) {
        self.commands.insert(cmd.name.clone(), cmd);
    }

    pub async fn get_command(&self, name: &str) -> Option<&Command> {
        self.commands.get(name)
    }

    pub async fn search_commands(&self, query: &str) -> Vec<&Command> {
        self.commands
            .values()
            .filter(|cmd| {
                cmd.name.contains(query) ||
                cmd.description.contains(query) ||
                cmd.syntax.contains(query)
            })
            .collect()
    }
}

pub async fn init_knowledge_base() -> Result<()> {
    info!("Initializing knowledge base...");
    
    let mut kb = KNOWLEDGE_BASE.write().await;
    
    // Add Nmap commands
    kb.add_command(Command {
        name: "nmap_basic".to_string(),
        description: "Basic port scanning with Nmap".to_string(),
        syntax: "nmap -sS -sV target.com".to_string(),
        examples: vec![
            "nmap -sS -sV 192.168.1.1".to_string(),
            "nmap -p- -A 10.0.0.1".to_string(),
            "nmap --script vuln target.com".to_string(),
        ],
        category: CommandCategory::Scanning,
        risk_level: RiskLevel::Low,
    }).await;

    // Add Metasploit commands
    kb.add_command(Command {
        name: "metasploit_handler".to_string(),
        description: "Setup Metasploit reverse handler".to_string(),
        syntax: "use exploit/multi/handler".to_string(),
        examples: vec![
            "set PAYLOAD windows/meterpreter/reverse_tcp".to_string(),
            "set LHOST 192.168.1.100".to_string(),
            "set LPORT 4444".to_string(),
            "exploit -j".to_string(),
        ],
        category: CommandCategory::Exploitation,
        risk_level: RiskLevel::High,
    }).await;

    info!("✅ Knowledge base initialized with {} commands", kb.commands.len());
    
    Ok(())
}

pub async fn get_knowledge_base() -> Arc<RwLock<KnowledgeBase>> {
    KNOWLEDGE_BASE.clone()
}
