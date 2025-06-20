use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::net::TcpStream;

#[derive(Debug, Clone, Copy, PartialEq,  Eq)]
pub enum PortStatus {
    Open,
    Closed,
    Filtered,
}

pub async fn scan_port(ip: IpAddr, port: u16, timeout: Duration, verbose: bool) -> PortStatus {
    let socket = SocketAddr::new(ip, port);
    if verbose {
        println!("[SCAN] Starting scan on {}:{}", socket.ip(), socket.port());
    }
    match tokio::time::timeout(timeout, TcpStream::connect(socket)).await {
        Ok(Ok(_stream)) => PortStatus::Open,
        Ok(Err(_)) => PortStatus::Closed,
        Err(_) => PortStatus::Filtered,
    }
}
