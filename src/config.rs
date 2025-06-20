use tokio::time::Duration;

pub struct ScanOptions {
    pub targets: Vec<String>,
    pub ports: Vec<u16>,
    pub timeout: Duration,
    // pub output_format: OutputFormat,
    pub parallelism: usize,
    pub open_only: bool,
}
