use clap::CommandFactory;
use clap::Parser;
use portscanx::cli::Cli;
use portscanx::config::ScanOptions;
use portscanx::scanner::run_scan;
use portscanx::updater::update;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.update {
        update().expect("Failed to update the application");
        return;
    } else if cli.version.is_some() {
        Cli::command().get_version().unwrap_or("Unknown version");
        return;
    }

    let _ = match &cli.target {
        Some(t) => t,
        None => {
            eprintln!("Error: missing required argument <IP/Hostname>");
            std::process::exit(1);
        }
    };

    let options = ScanOptions::from(cli);
    run_scan(options).await;
}
