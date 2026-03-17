//! Command Database
//! Stores and manages ethical hacking commands

use super::{Command, CommandCategory, RiskLevel, KnowledgeBase};
use anyhow::Result;
use std::collections::HashMap;

pub struct CommandDatabase {
    commands: HashMap<String, Command>,
}

impl CommandDatabase {
    pub fn new() -> Self {
        let mut commands = HashMap::new();
        
        // Nmap commands
        commands.insert("nmap_syn_scan".to_string(), Command {
            name: "nmap_syn_scan".to_string(),
            description: "SYN scan for fast port discovery".to_string(),
            syntax: "nmap -sS -p- target.com".to_string(),
            examples: vec![
                "nmap -sS -p 1-1000 192.168.1.1".to_string(),
                "nmap -sS -p- --min-rate 5000 10.0.0.1".to_string(),
            ],
            category: CommandCategory::Scanning,
            risk_level: RiskLevel::Low,
        });

        commands.insert("nmap_version_scan".to_string(), Command {
            name: "nmap_version_scan".to_string(),
            description: "Service version detection".to_string(),
            syntax: "nmap -sV -p 80,443 target.com".to_string(),
            examples: vec![
                "nmap -sV --version-intensity 5 192.168.1.1".to_string(),
                "nmap -sV -sC 10.0.0.1".to_string(),
            ],
            category: CommandCategory::Enumeration,
            risk_level: RiskLevel::Low,
        });

        commands.insert("nmap_script_scan".to_string(), Command {
            name: "nmap_script_scan".to_string(),
            description: "Run NSE scripts for vulnerability detection".to_string(),
            syntax: "nmap --script vuln target.com".to_string(),
            examples: vec![
                "nmap --script http-enum 192.168.1.1".to_string(),
                "nmap --script smb-vuln* 10.0.0.1".to_string(),
            ],
            category: CommandCategory::Enumeration,
            risk_level: RiskLevel::Medium,
        });

        // Metasploit commands
        commands.insert("msf_console".to_string(), Command {
            name: "msf_console".to_string(),
            description: "Start Metasploit console".to_string(),
            syntax: "msfconsole".to_string(),
            examples: vec![
                "msfconsole -q".to_string(),
                "msfconsole -r script.rc".to_string(),
            ],
            category: CommandCategory::Exploitation,
            risk_level: RiskLevel::Medium,
        });

        commands.insert("msf_search".to_string(), Command {
            name: "msf_search".to_string(),
            description: "Search for exploits".to_string(),
            syntax: "search <query>".to_string(),
            examples: vec![
                "search type:exploit platform:windows".to_string(),
                "search cve:2021".to_string(),
                "search eternalblue".to_string(),
            ],
            category: CommandCategory::Exploitation,
            risk_level: RiskLevel::Low,
        });

        commands.insert("msf_use".to_string(), Command {
            name: "msf_use".to_string(),
            description: "Use a specific exploit".to_string(),
            syntax: "use <exploit_path>".to_string(),
            examples: vec![
                "use exploit/windows/smb/ms17_010_eternalblue".to_string(),
                "use auxiliary/scanner/portscan/tcp".to_string(),
            ],
            category: CommandCategory::Exploitation,
            risk_level: RiskLevel::Medium,
        });

        // SQLMap commands
        commands.insert("sqlmap_basic".to_string(), Command {
            name: "sqlmap_basic".to_string(),
            description: "Basic SQL injection testing".to_string(),
            syntax: "sqlmap -u 'http://target.com/page?id=1'".to_string(),
            examples: vec![
                "sqlmap -u 'http://target.com/page?id=1' --dbs".to_string(),
                "sqlmap -u 'http://target.com/page?id=1' -D dbname --tables".to_string(),
                "sqlmap -r request.txt -p parameter".to_string(),
            ],
            category: CommandCategory::Exploitation,
            risk_level: RiskLevel::High,
        });

        // Hydra commands
        commands.insert("hydra_bruteforce".to_string(), Command {
            name: "hydra_bruteforce".to_string(),
            description: "Password bruteforce attacks".to_string(),
            syntax: "hydra -l admin -P passwords.txt target.com ssh".to_string(),
            examples: vec![
                "hydra -L users.txt -P pass.txt ftp://192.168.1.1".to_string(),
                "hydra -l admin -P rockyou.txt http-post-form '/login:user=^USER^&pass=^PASS^:F=incorrect'".to_string(),
            ],
            category: CommandCategory::Exploitation,
            risk_level: RiskLevel::High,
        });

        // WPScan commands
        commands.insert("wpscan_basic".to_string(), Command {
            name: "wpscan_basic".to_string(),
            description: "WordPress vulnerability scanner".to_string(),
            syntax: "wpscan --url http://target.com".to_string(),
            examples: vec![
                "wpscan --url http://target.com --enumerate u".to_string(),
                "wpscan --url http://target.com --api-token API_KEY".to_string(),
            ],
            category: CommandCategory::Enumeration,
            risk_level: RiskLevel::Low,
        });

        Self { commands }
    }

    pub fn search(&self, query: &str) -> Vec<&Command> {
        let query = query.to_lowercase();
        self.commands
            .values()
            .filter(|cmd| {
                cmd.name.to_lowercase().contains(&query) ||
                cmd.description.to_lowercase().contains(&query) ||
                cmd.syntax.to_lowercase().contains(&query)
            })
            .collect()
    }

    pub fn get_by_category(&self, category: CommandCategory) -> Vec<&Command> {
        self.commands
            .values()
            .filter(|cmd| cmd.category == category)
            .collect()
    }

    pub fn get_by_risk(&self, risk_level: RiskLevel) -> Vec<&Command> {
        self.commands
            .values()
            .filter(|cmd| cmd.risk_level == risk_level)
            .collect()
    }

    pub fn get_command(&self, name: &str) -> Option<&Command> {
        self.commands.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_search() {
        let db = CommandDatabase::new();
        let results = db.search("nmap");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_category_filter() {
        let db = CommandDatabase::new();
        let scanning = db.get_by_category(CommandCategory::Scanning);
        assert!(!scanning.is_empty());
    }
}
