use clap::CommandFactory;
use clap::Parser;
use portscanx::cli::Cli;
use portscanx::config::ScanOptions;
use portscanx::scanner::run_scan;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if cli.version == Some(true) {
        println!(
            "{}",
            Cli::command().get_version().unwrap_or("Unknown version")
        );
        return;
    }
    let options = ScanOptions::from(cli);
    run_scan(options).await;
}
