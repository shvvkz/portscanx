use std::net::IpAddr;
use ipnet::IpNet;

pub fn expand_ips(targets: &Vec<String>) -> Vec<IpAddr> {
    let mut result = Vec::new();

    for target in targets {
        if let Ok(ip) = target.parse::<IpAddr>() {
            result.push(ip);
        } else if let Ok(cidr) = target.parse::<IpNet>() {
            result.extend(cidr.hosts());
        }
    }

    result
}
