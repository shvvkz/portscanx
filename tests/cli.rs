use assert_cmd::Command;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("portscanx").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("Usage"));
}

#[test]
fn test_cli_run_localhost() {
    let mut cmd = Command::cargo_bin("portscanx").unwrap();
    cmd.args(["127.0.0.1", "--ports", "80", "--timeout", "100"]);
    cmd.assert().success();
}

#[test]
fn test_cli_run_invalid_ip() {
    let mut cmd = Command::cargo_bin("portscanx").unwrap();
    cmd.args(["999.999.999.999", "--ports", "80", "--timeout", "100"]);
    cmd.assert().failure().stderr(predicates::str::contains(
        "No valid IP addresses or CIDR ranges found in the targets.",
    ));
}
