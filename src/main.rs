use portscanx::cli::Cli;
use portscanx::scanner::run_scan;
use portscanx::config::ScanOptions;
use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let options = ScanOptions::from(cli);
    run_scan(options).await;
}
