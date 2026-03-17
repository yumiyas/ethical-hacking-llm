use ethical_hacking_llm::security::validator::InputValidator;
use ethical_hacking_llm::security::rate_limiter::RateLimiter;

#[test]
fn test_input_validation() {
    let validator = InputValidator::new();
    
    // Valid input
    assert!(validator.validate("nmap -sV target.com").is_ok());
    
    // Invalid - too long
    let long_input = "a".repeat(2000);
    assert!(validator.validate(&long_input).is_err());
    
    // Invalid - blocked pattern
    assert!(validator.validate("rm -rf /").is_err());
}

#[tokio::test]
async fn test_rate_limiter() {
    let limiter = RateLimiter::new(5, 60);
    
    // 5 requests should succeed
    for i in 0..5 {
        assert!(limiter.check_rate_limit("test").await);
    }
    
    // 6th should fail
    assert!(!limiter.check_rate_limit("test").await);
    
    // Different key should work
    assert!(limiter.check_rate_limit("other").await);
}
