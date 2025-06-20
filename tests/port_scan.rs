use std::time::Duration;
use tokio::net::TcpListener;
use portscanx::scanner::port::{scan_port, PortStatus};

#[tokio::test]
async fn test_port_open() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let result = scan_port("127.0.0.1".parse().unwrap(), port, Duration::from_millis(300), false).await;
    assert_eq!(result, PortStatus::Open);
}

#[tokio::test]
async fn test_port_closed() {
    let result = scan_port("127.0.0.1".parse().unwrap(), 65534, Duration::from_millis(300), false).await;
    assert!(matches!(result, PortStatus::Closed | PortStatus::Filtered));
}
