use std::process::Command;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const REPO: &str = "shvvkz/portscanx";
const BINARY_PATH: &str = "/usr/local/bin/portscanx";

pub fn update() -> Result<(), Box<dyn std::error::Error>> {
    let latest_version_output = Command::new("curl")
        .args([
            "-s",
            &format!("https://api.github.com/repos/{}/releases/latest", REPO),
        ])
        .output()?;

    let stdout = String::from_utf8_lossy(&latest_version_output.stdout);
    let tag = stdout
        .lines()
        .find(|line| line.contains("\"tag_name\""))
        .and_then(|line| line.split('"').nth(3))
        .ok_or("Failed to get latest version")?;

    if tag != format!("v{}", VERSION) {
        println!("⬇️ New version {} found, updating...", tag);

        let tmp_path = "/tmp/hyprland-shortcuts";
        Command::new("curl")
            .args([
                "-L",
                "-o",
                tmp_path,
                &format!(
                    "https://github.com/{}/releases/download/{}/hyprland-shortcuts",
                    REPO, tag
                ),
            ])
            .status()?;

        Command::new("sudo")
            .args(["mv", tmp_path, &BINARY_PATH])
            .status()?;
        Command::new("sudo")
            .args(["chmod", "+x", &BINARY_PATH])
            .status()?;

        println!("Updated to version {}", tag);
    } else {
        println!("Already up to date (v{})", VERSION);
    }
    Ok(())
}