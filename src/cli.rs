use clap::Parser;

use crate::{
    config::{OutputFormat, ScanOptions},
    scanner::service::parse_ports,
};

#[derive(Parser, Debug)]
#[command(
    name = "portscanx",
    version,
    disable_version_flag = true,
    about = "A fast and flexible port scanner written in Rust."
)]
pub struct Cli {
    /// The IP address or hostname to scan.
    #[arg(value_name = "IP/Hostname")]
    pub target: String,

    /// The port or range of ports to scan (e.g., 80, 1-1000).
    #[arg(short, long, value_name = "PORT/RANGE", default_value = "1-65535")]
    pub ports: String,

    /// Timeout for each port scan in milliseconds.
    #[arg(short, long, default_value_t = 500)]
    pub timeout: u64,

    /// Output format: terminal, json, csv
    #[arg(short, long, default_value = "terminal")]
    pub output: String,

    /// Only show open ports.
    #[arg(long)]
    pub only_open: bool,

    /// Enable verbose output.
    #[arg(short, long)]
    pub verbose: bool,

    #[arg(long)]
    /// Show version information and exit.
    #[arg(long = "version", action = clap::ArgAction::Version)]
    pub version: Option<bool>,
}

impl From<Cli> for ScanOptions {
    fn from(cli: Cli) -> Self {
        let targets = vec![cli.target];
        let ports = parse_ports(&cli.ports);

        let output_format = match cli.output.as_str() {
            "json" => OutputFormat::Json,
            "csv" => OutputFormat::Csv,
            _ => OutputFormat::Terminal,
        };

        ScanOptions {
            targets,
            ports,
            timeout: std::time::Duration::from_millis(cli.timeout),
            verbose: cli.verbose,
            output_format,
            parallelism: 100, // ou tu peux ajouter une option plus tard
            open_only: cli.only_open,
        }
    }
}
