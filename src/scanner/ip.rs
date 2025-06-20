use ipnet::IpNet;
use std::net::IpAddr;

pub fn expand_ips(targets: &Vec<String>) -> Vec<IpAddr> {
    let mut result = Vec::new();

    for target in targets {
        if let Ok(ip) = target.parse::<IpAddr>() {
            result.push(ip);
        } else if let Ok(cidr) = target.parse::<IpNet>() {
            result.extend(cidr.hosts());
        }
    }
    if result.is_empty() {
        eprintln!("No valid IP addresses or CIDR ranges found in the targets.");
        std::process::exit(1);
    }
    result
}
