use crate::scanner::ScanResultHandler;

pub fn output_csv(results: ScanResultHandler, only_open: bool) -> String {
    let mut output = String::new();
    output.push_str("IP,Port,Status,Service\n");

    for (ip, port) in results.open {
        let service = crate::scanner::service::get_service_name(port).unwrap_or("unknown");
        output.push_str(&format!("{},{},open,{}\n", ip, port, service));
    }

    if !only_open {
        for (ip, port) in results.filtered {
            output.push_str(&format!("{},{},filtered,\n", ip, port));
        }
        for (ip, port) in results.closed {
            output.push_str(&format!("{},{},closed,\n", ip, port));
        }
    }

    output
}