pub mod ip;
pub mod port;
pub mod service;

use crate::config::ScanOptions;
use crate::scanner::{ip::expand_ips, port::scan_port};
use owo_colors::OwoColorize;
use std::net::IpAddr;

use futures::stream::{FuturesUnordered, StreamExt};
use tokio::task;

pub async fn run_scan(options: ScanOptions) {
    let t0 = std::time::Instant::now();
    let ips = expand_ips(&options.targets);
    let mut tasks = FuturesUnordered::new();

    for ip in ips {
        for &port in &options.ports {
            tasks.push(task::spawn(async move {
                let is_open = scan_port(ip, port, options.timeout, options.verbose).await;
                (ip, port, is_open)
            }));
        }
    }

    let mut results = ScanResultHandler::new();

    while let Some(Ok((ip, port, port_status))) = tasks.next().await {
        results.add_result(ip, port, port_status);
    }
    println!(
        "{} {} {:.3}s",
        "✅".green(),
        "Scan finished in".bold(),
        t0.elapsed().as_secs_f32()
    );

    println!(
        "{}\n{}",
        "⚠️  Service names may not be accurate.".bold().yellow(),
        "They are based only on port numbers, not on service detection.".dimmed()
    );

    if options.output_format == crate::config::OutputFormat::Json {
        let json_output = crate::output::json::output_json(results, options.open_only);
        std::fs::write("portscanx.json", json_output).expect("Failed to write JSON output");
    } else if options.output_format == crate::config::OutputFormat::Csv {
        let csv_output = crate::output::csv::output_csv(results, options.open_only);
        std::fs::write("portscanx.csv", csv_output).expect("Failed to write CSV output");
    } else if options.output_format == crate::config::OutputFormat::Terminal {
        crate::output::terminal::output_terminal(results, options.open_only);
    }
}

pub struct ScanResultHandler {
    pub open: Vec<(IpAddr, u16)>,
    pub filtered: Vec<(IpAddr, u16)>,
    pub closed: Vec<(IpAddr, u16)>,
}

impl ScanResultHandler {
    pub fn new() -> Self {
        ScanResultHandler {
            open: Vec::new(),
            filtered: Vec::new(),
            closed: Vec::new(),
        }
    }

    pub fn add_result(&mut self, ip: IpAddr, port: u16, status: port::PortStatus) {
        match status {
            port::PortStatus::Open => self.open.push((ip, port)),
            port::PortStatus::Filtered => self.filtered.push((ip, port)),
            port::PortStatus::Closed => self.closed.push((ip, port)),
        }
    }
}

impl Default for ScanResultHandler {
    fn default() -> Self {
        Self::new()
    }
}
