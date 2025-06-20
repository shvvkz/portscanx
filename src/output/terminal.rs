use crate::scanner::ScanResultHandler;
use crate::scanner::service::get_service_name;
use owo_colors::OwoColorize;

pub fn output_terminal(results: ScanResultHandler, only_open: bool) {
    for (ip, port) in &results.open {
        let service = get_service_name(*port).unwrap_or("unknown");
        println!(
            "{} {ip}:{port} → {} ({service})",
            "✔".green(),
            "open".bold().green()
        );
    }
    if !only_open {
        for (ip, port) in &results.filtered {
            println!("{} {ip}:{port} → {}", "✘".red(), "closed".dimmed());
        }

        for (ip, port) in &results.closed {
            println!("{} {ip}:{port} → {}", "?".yellow(), "filtered".bold().yellow()
            );
        }
    }
}
