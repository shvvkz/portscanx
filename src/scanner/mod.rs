pub mod ip;
pub mod service;
pub mod port;

use crate::config::ScanOptions;
use crate::scanner::{ip::expand_ips, port::scan_port, service::get_service_name};

use tokio::task;
use futures::stream::{FuturesUnordered, StreamExt};

pub async fn run_scan(options: ScanOptions) {
    let ips = expand_ips(&options.targets);
    let mut tasks = FuturesUnordered::new();

    for ip in ips {
        for &port in &options.ports {
            let timeout = options.timeout;
            let ip_copy = ip.clone();
            tasks.push(task::spawn(async move {
                let is_open = scan_port(ip_copy, port, timeout).await;
                (ip_copy, port, is_open)
            }));
        }
    }

    let mut results = Vec::new();

    while let Some(Ok((ip, port, port_status))) = tasks.next().await {
        results.push((ip, port, port_status));
    }

    println!("\n✅ Scan terminé. Résultats :\n");

    for (ip, port, port_status) in &results {
        if *port_status == port::PortStatus::Open {
            let service = get_service_name(*port).unwrap_or("unknown");
            println!("✔ {ip}:{port} → open ({service})");
        } else if !options.open_only && *port_status == port::PortStatus::Filtered {
            println!("✘ {ip}:{port} → filtered");
        } else if !options.open_only && *port_status == port::PortStatus::Closed {
            println!("✘ {ip}:{port} → closed");
        }
    }
}
