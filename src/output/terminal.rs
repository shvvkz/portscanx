use crate::scanner::service::get_service_name;
use crate::scanner::ScanResultHandler;

pub fn output_terminal(results: ScanResultHandler, only_open: bool){
    for (ip, port) in &results.open {
        let service = get_service_name(*port).unwrap_or("unknown");
        println!("✔ {ip}:{port} → open ({service})");
    }
    if !only_open {
        for (ip, port) in &results.filtered {
            println!("❗{ip}:{port} → filtered");
        }

        for (ip, port) in &results.closed {
            println!("✖ {ip}:{port} → closed");
        }
    }
}
