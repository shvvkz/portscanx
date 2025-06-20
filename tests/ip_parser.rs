use portscanx::scanner::ip::expand_ips;

#[test]
fn test_expand_single_ip() {
    let input = vec!["192.168.1.1".to_string()];
    let result = expand_ips(&input);
    assert_eq!(result[0].to_string(), "192.168.1.1");
}

#[test]
fn test_expand_cidr() {
    let input = vec!["192.168.1.0/30".to_string()];
    let result = expand_ips(&input);
    assert_eq!(result.len(), 2);
}
