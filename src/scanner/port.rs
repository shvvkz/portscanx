use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::net::TcpStream;

pub enum PortStatus {
    Open,
    Closed,
    Filtered,
}

impl PartialEq for PortStatus {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), (PortStatus::Open, PortStatus::Open))
    }
}

pub async fn scan_port(ip: IpAddr, port: u16, timeout: Duration) -> PortStatus {
    let socket = SocketAddr::new(ip, port);
    println!("Scanning {}:{}", socket.ip(), socket.port());
    match tokio::time::timeout(timeout, TcpStream::connect(socket)).await {
        Ok(Ok(_stream)) => {
            println!("[scan_port] ✔ {}:{} → open", socket.ip(), socket.port());
            PortStatus::Open
        },
        Ok(Err(_)) => {
            println!("[scan_port] ✘ {}:{} → closed", socket.ip(), socket.port());
            PortStatus::Closed
        },
        Err(_) => {
            println!("[scan_port] ? {}:{} → filtered (timeout)", socket.ip(), socket.port());
            PortStatus::Filtered
        },
    }
}
