//! Tools Database
//! Information about ethical hacking tools

use super::Tool;
use std::collections::HashMap;

pub struct ToolDatabase {
    tools: HashMap<String, Tool>,
}

impl ToolDatabase {
    pub fn new() -> Self {
        let mut tools = HashMap::new();

        // Nmap
        tools.insert("nmap".to_string(), Tool {
            name: "Nmap".to_string(),
            description: "Network discovery and security scanning tool".to_string(),
            installation: "sudo apt-get install nmap".to_string(),
            basic_usage: "nmap -sV target.com".to_string(),
            examples: vec![
                "nmap -sS -sV -p- 192.168.1.1".to_string(),
                "nmap --script vuln target.com".to_string(),
                "nmap -sn 192.168.1.0/24".to_string(),
            ],
            category: "Scanner".to_string(),
        });

        // Metasploit
        tools.insert("metasploit".to_string(), Tool {
            name: "Metasploit Framework".to_string(),
            description: "Penetration testing framework for exploit development".to_string(),
            installation: "curl https://raw.githubusercontent.com/rapid7/metasploit-omnibus/master/config/templates/metasploit-framework-wrappers/msfupdate.erb > msfinstall && chmod 755 msfinstall && ./msfinstall".to_string(),
            basic_usage: "msfconsole".to_string(),
            examples: vec![
                "use exploit/multi/handler".to_string(),
                "set PAYLOAD windows/meterpreter/reverse_tcp".to_string(),
                "set LHOST 192.168.1.100".to_string(),
            ],
            category: "Framework".to_string(),
        });

        // Burp Suite
        tools.insert("burpsuite".to_string(), Tool {
            name: "Burp Suite".to_string(),
            description: "Web application security testing platform".to_string(),
            installation: "Download from portswigger.net".to_string(),
            basic_usage: "Configure browser proxy to localhost:8080".to_string(),
            examples: vec![
                "Intercept HTTP requests".to_string(),
                "Use Intruder for brute force".to_string(),
                "Repeater for manual testing".to_string(),
            ],
            category: "Web".to_string(),
        });

        // SQLMap
        tools.insert("sqlmap".to_string(), Tool {
            name: "SQLMap".to_string(),
            description: "Automatic SQL injection tool".to_string(),
            installation: "sudo apt-get install sqlmap".to_string(),
            basic_usage: "sqlmap -u 'http://target.com/page?id=1'".to_string(),
            examples: vec![
                "sqlmap -u 'http://target.com/page?id=1' --dbs".to_string(),
                "sqlmap -r request.txt --level=5 --risk=3".to_string(),
                "sqlmap -u 'http://target.com/page?id=1' --os-shell".to_string(),
            ],
            category: "Web".to_string(),
        });

        // Wireshark
        tools.insert("wireshark".to_string(), Tool {
            name: "Wireshark".to_string(),
            description: "Network protocol analyzer".to_string(),
            installation: "sudo apt-get install wireshark".to_string(),
            basic_usage: "sudo wireshark".to_string(),
            examples: vec![
                "Capture traffic on interface eth0".to_string(),
                "Filter by HTTP requests".to_string(),
                "Follow TCP streams".to_string(),
            ],
            category: "Network".to_string(),
        });

        // Hydra
        tools.insert("hydra".to_string(), Tool {
            name: "Hydra".to_string(),
            description: "Password brute-forcing tool".to_string(),
            installation: "sudo apt-get install hydra".to_string(),
            basic_usage: "hydra -l admin -P passwords.txt target.com ssh".to_string(),
            examples: vec![
                "hydra -L users.txt -P pass.txt ftp://192.168.1.1".to_string(),
                "hydra -l admin -P rockyou.txt http-post-form '/login:user=^USER^&pass=^PASS^:F=incorrect'".to_string(),
            ],
            category: "Password".to_string(),
        });

        // John the Ripper
        tools.insert("john".to_string(), Tool {
            name: "John the Ripper".to_string(),
            description: "Password cracking tool".to_string(),
            installation: "sudo apt-get install john".to_string(),
            basic_usage: "john --wordlist=rockyou.txt hash.txt".to_string(),
            examples: vec![
                "john --format=nt hash.txt".to_string(),
                "john --rules --wordlist=dictionary.txt hash.txt".to_string(),
                "john --show hash.txt".to_string(),
            ],
            category: "Password".to_string(),
        });

        // Hashcat
        tools.insert("hashcat".to_string(), Tool {
            name: "Hashcat".to_string(),
            description: "Advanced password recovery tool with GPU support".to_string(),
            installation: "sudo apt-get install hashcat".to_string(),
            basic_usage: "hashcat -m 0 -a 0 hash.txt rockyou.txt".to_string(),
            examples: vec![
                "hashcat -m 1000 -a 3 hash.txt ?a?a?a?a".to_string(),
                "hashcat -m 0 -a 6 hash.txt dictionary.txt ?d?d".to_string(),
            ],
            category: "Password".to_string(),
        });

        // Gobuster
        tools.insert("gobuster".to_string(), Tool {
            name: "Gobuster".to_string(),
            description: "Directory/file brute-forcing tool".to_string(),
            installation: "sudo apt-get install gobuster".to_string(),
            basic_usage: "gobuster dir -u http://target.com -w wordlist.txt".to_string(),
            examples: vec![
                "gobuster dir -u http://target.com -w common.txt -x php,html,txt".to_string(),
                "gobuster dns -d target.com -w subdomains.txt".to_string(),
            ],
            category: "Web".to_string(),
        });

        Self { tools }
    }

    pub fn search(&self, query: &str) -> Vec<&Tool> {
        let query = query.to_lowercase();
        self.tools
            .values()
            .filter(|t| {
                t.name.to_lowercase().contains(&query) ||
                t.description.to_lowercase().contains(&query) ||
                t.category.to_lowercase().contains(&query)
            })
            .collect()
    }

    pub fn get_by_category(&self, category: &str) -> Vec<&Tool> {
        self.tools
            .values()
            .filter(|t| t.category.to_lowercase() == category.to_lowercase())
            .collect()
    }

    pub fn get_tool(&self, name: &str) -> Option<&Tool> {
        self.tools.get(name)
    }

    pub fn get_all(&self) -> Vec<&Tool> {
        self.tools.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_search() {
        let db = ToolDatabase::new();
        let results = db.search("nmap");
        assert!(!results.is_empty());
        assert_eq!(results[0].name, "Nmap");
    }

    #[test]
    fn test_category_filter() {
        let db = ToolDatabase::new();
        let web_tools = db.get_by_category("web");
        assert!(!web_tools.is_empty());
    }
}
