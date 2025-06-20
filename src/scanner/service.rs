use std::collections::HashMap;

pub fn get_service_name(port: u16) -> Option<&'static str> {
    let map = known_services();
    map.get(&port).copied()
}

pub fn parse_ports(input: &str) -> Vec<u16> {
    if let Some((start, end)) = input.split_once('-') {
        let start = start.trim().parse().unwrap_or(1);
        let end = end.trim().parse().unwrap_or(65535);
        (start..=end).collect()
    } else if let Ok(single) = input.trim().parse() {
        vec![single]
    } else {
        vec![]
    }
}

fn known_services() -> HashMap<u16, &'static str> {
    HashMap::from([
        (7, "echo"),
        (9, "discard"),
        (11, "systat"),
        (13, "daytime"),
        (17, "qotd"),
        (19, "chargen"),
        (20, "ftp-data"),
        (21, "ftp"),
        (22, "ssh"),
        (23, "telnet"),
        (25, "smtp"),
        (37, "time"),
        (42, "nameserver"),
        (53, "dns"),
        (67, "dhcp-server"),
        (68, "dhcp-client"),
        (69, "tftp"),
        (79, "finger"),
        (80, "http"),
        (110, "pop3"),
        (111, "rpcbind"),
        (113, "ident"),
        (119, "nntp"),
        (123, "ntp"),
        (143, "imap"),
        (161, "snmp"),
        (162, "snmptrap"),
        (179, "bgp"),
        (194, "irc"),
        (443, "https"),
        (445, "microsoft-ds"),
        (465, "smtps"),
        (514, "syslog"),
        (515, "printer"),
        (520, "ripv1"),
        (521, "ripv2"),
        (548, "afp"),
        (587, "submission"),
        (631, "ipp"),
        (636, "ldaps"),
        (873, "rsync"),
        (993, "imaps"),
        (995, "pop3s"),
        (1080, "socks"),
        (1433, "ms-sql-s"),
        (1521, "oracle-db"),
        (2049, "nfs"),
        (2181, "zookeeper"),
        (2375, "docker"),
        (2376, "docker-tls"),
        (27017, "mongodb"),
        (28017, "mongodb-web"),
        (3128, "squid"),
        (3306, "mysql"),
        (5432, "postgres"),
        (6379, "redis"),
    ])
}
