use portscanx::scanner::run_scan;
use portscanx::config::ScanOptions;

#[tokio::main]
async fn main() {
    let options = ScanOptions {
        targets: vec!["192.168.100.1".to_string()],
        ports: vec![22, 80, 443, 8080, 3306, 5432, 6379, 65535, 1],
        output_format: portscanx::config::OutputFormat::Terminal,
        verbose: false,
        timeout: std::time::Duration::from_millis(500),
        parallelism: 100,
        open_only: true,
    };

    run_scan(options).await;
}
