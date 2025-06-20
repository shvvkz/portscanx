use crate::scanner::ScanResultHandler;

pub fn output_json(results: ScanResultHandler, only_open: bool) -> String {
    let mut output = JsonOutput {
        ip: IpOutput { ports: Vec::new() },
    };
    for (_, port) in results.open {
        let service = crate::scanner::service::get_service_name(port).unwrap_or("unknown");
        output.ip.ports.push(PortOutput {
            port,
            status: "open".to_string(),
            service: Some(service.to_string()),
        });
    }
    if !only_open {
        for (_, port) in results.filtered {
            output.ip.ports.push(PortOutput {
                port,
                status: "filtered".to_string(),
                service: None,
            });
        }
        for (_, port) in results.closed {
            output.ip.ports.push(PortOutput {
                port,
                status: "closed".to_string(),
                service: None,
            });
        }
    }
    serde_json::to_string_pretty(&output)
        .unwrap_or_else(|_| String::from("Error generating JSON output"))
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct JsonOutput {
    pub ip: IpOutput,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct IpOutput {
    pub ports: Vec<PortOutput>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PortOutput {
    pub port: u16,
    pub status: String,
    pub service: Option<String>,
}
