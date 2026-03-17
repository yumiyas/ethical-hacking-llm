//! Techniques Database
//! Stores ethical hacking techniques with MITRE ATT&CK mapping

use super::Technique;
use std::collections::HashMap;

pub struct TechniqueDatabase {
    techniques: HashMap<String, Technique>,
}

impl TechniqueDatabase {
    pub fn new() -> Self {
        let mut techniques = HashMap::new();

        // SQL Injection
        techniques.insert("sql_injection".to_string(), Technique {
            name: "SQL Injection".to_string(),
            description: "Inject SQL queries through input fields to manipulate database queries".to_string(),
            mitre_id: Some("T1190".to_string()),
            steps: vec![
                "Identify input fields that interact with database".to_string(),
                "Test with single quote (') to trigger errors".to_string(),
                "Use UNION queries to extract data".to_string(),
                "Bypass filters with encoding".to_string(),
                "Extract database structure and data".to_string(),
            ],
            tools: vec![
                "sqlmap".to_string(),
                "Burp Suite".to_string(),
                "OWASP ZAP".to_string(),
            ],
            mitigation: "Use prepared statements, input validation, and WAF".to_string(),
        });

        // XSS
        techniques.insert("xss".to_string(), Technique {
            name: "Cross-Site Scripting (XSS)".to_string(),
            description: "Inject client-side scripts into web pages viewed by other users".to_string(),
            mitre_id: Some("T1059".to_string()),
            steps: vec![
                "Find reflection points (URL parameters, forms)".to_string(),
                "Inject script tags (<script>alert(1)</script>)".to_string(),
                "Bypass filters with event handlers".to_string(),
                "Steal cookies/session tokens".to_string(),
                "Perform actions on behalf of user".to_string(),
            ],
            tools: vec![
                "XSStrike".to_string(),
                "BeEF".to_string(),
                "Burp Suite".to_string(),
            ],
            mitigation: "Input sanitization, CSP headers, HttpOnly cookies".to_string(),
        });

        // Buffer Overflow
        techniques.insert("buffer_overflow".to_string(), Technique {
            name: "Buffer Overflow".to_string(),
            description: "Exploit buffer overflows to execute arbitrary code".to_string(),
            mitre_id: Some("T1200".to_string()),
            steps: vec![
                "Identify vulnerable program".to_string(),
                "Fuzz to find crash point".to_string(),
                "Control EIP/RIP".to_string(),
                "Find bad characters".to_string(),
                "Find return address".to_string(),
                "Generate and inject shellcode".to_string(),
            ],
            tools: vec![
                "Immunity Debugger".to_string(),
                "msfvenom".to_string(),
                "pattern_create.rb".to_string(),
            ],
            mitigation: "Stack canaries, ASLR, DEP, code reviews".to_string(),
        });

        // Man-in-the-Middle
        techniques.insert("mitm".to_string(), Technique {
            name: "Man-in-the-Middle".to_string(),
            description: "Intercept and modify network traffic".to_string(),
            mitre_id: Some("T1557".to_string()),
            steps: vec![
                "ARP spoofing on local network".to_string(),
                "DNS spoofing".to_string(),
                "Set up proxy".to_string(),
                "Capture traffic".to_string(),
                "Modify packets".to_string(),
            ],
            tools: vec![
                "Ettercap".to_string(),
                "Bettercap".to_string(),
                "Wireshark".to_string(),
            ],
            mitigation: "Use encryption (HTTPS), ARP spoofing detection".to_string(),
        });

        // Phishing
        techniques.insert("phishing".to_string(), Technique {
            name: "Phishing".to_string(),
            description: "Social engineering to steal credentials".to_string(),
            mitre_id: Some("T1566".to_string()),
            steps: vec![
                "Clone legitimate website".to_string(),
                "Set up email campaign".to_string(),
                "Craft convincing message".to_string(),
                "Host phishing page".to_string(),
                "Collect credentials".to_string(),
            ],
            tools: vec![
                "Social Engineering Toolkit".to_string(),
                "GoPhish".to_string(),
                "Evilginx2".to_string(),
            ],
            mitigation: "User awareness training, MFA, email filtering".to_string(),
        });

        // Privilege Escalation
        techniques.insert("privesc_linux".to_string(), Technique {
            name: "Linux Privilege Escalation".to_string(),
            description: "Escalate privileges on Linux systems".to_string(),
            mitre_id: Some("T1068".to_string()),
            steps: vec![
                "Enumerate system (kernel, services, SUID)".to_string(),
                "Check sudo permissions".to_string(),
                "Look for vulnerable versions".to_string(),
                "Exploit misconfigurations".to_string(),
                "Gain root access".to_string(),
            ],
            tools: vec![
                "LinPEAS".to_string(),
                "LinEnum".to_string(),
                                "linux-exploit-suggester".to_string(),
            ],
            mitigation: "Regular updates, least privilege principle, proper configuration".to_string(),
        });

        // Password Cracking
        techniques.insert("password_cracking".to_string(), Technique {
            name: "Password Cracking".to_string(),
            description: "Crack password hashes using various methods".to_string(),
            mitre_id: Some("T1110".to_string()),
            steps: vec![
                "Obtain password hashes".to_string(),
                "Identify hash type".to_string(),
                "Choose attack method (dictionary, brute force, rule-based)".to_string(),
                "Use wordlists and rules".to_string(),
                "Optimize with GPU".to_string(),
            ],
            tools: vec![
                "John the Ripper".to_string(),
                "Hashcat".to_string(),
                "Hydra".to_string(),
            ],
            mitigation: "Strong password policy, MFA, account lockout".to_string(),
        });

        Self { techniques }
    }

    pub fn search(&self, query: &str) -> Vec<&Technique> {
        let query = query.to_lowercase();
        self.techniques
            .values()
            .filter(|t| {
                t.name.to_lowercase().contains(&query) ||
                t.description.to_lowercase().contains(&query) ||
                t.mitre_id.as_ref().map(|id| id.contains(&query)).unwrap_or(false)
            })
            .collect()
    }

    pub fn get_by_mitre_id(&self, mitre_id: &str) -> Option<&Technique> {
        self.techniques
            .values()
            .find(|t| t.mitre_id.as_ref().map(|id| id == mitre_id).unwrap_or(false))
    }

    pub fn get_technique(&self, name: &str) -> Option<&Technique> {
        self.techniques.get(name)
    }

    pub fn get_all(&self) -> Vec<&Technique> {
        self.techniques.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_technique_search() {
        let db = TechniqueDatabase::new();
        let results = db.search("sql");
        assert!(!results.is_empty());
        assert_eq!(results[0].name, "SQL Injection");
    }

    #[test]
    fn test_mitre_id_lookup() {
        let db = TechniqueDatabase::new();
        let technique = db.get_by_mitre_id("T1190");
        assert!(technique.is_some());
        assert_eq!(technique.unwrap().name, "SQL Injection");
    }
}
