use assert_cmd::Command;

#[test]
fn cli_shows_help() {
    Command::cargo_bin("chip")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("CHIP payment gateway CLI"));
}

#[test]
fn cli_shows_version() {
    Command::cargo_bin("chip")
        .unwrap()
        .arg("--version")
        .assert()
        .success();
}

#[test]
fn cli_errors_without_token() {
    Command::cargo_bin("chip")
        .unwrap()
        .env_remove("CHIP_API_TOKEN")
        .args(["get-purchase", "abc-123"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("token"));
}

#[test]
fn cli_errors_without_subcommand() {
    Command::cargo_bin("chip")
        .unwrap()
        .assert()
        .failure();
}

#[test]
fn cli_create_purchase_requires_flags() {
    Command::cargo_bin("chip")
        .unwrap()
        .env("CHIP_API_TOKEN", "test")
        .args(["create-purchase"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("--brand-id"));
}

#[test]
fn cli_verify_signature_requires_content_source() {
    Command::cargo_bin("chip")
        .unwrap()
        .args([
            "verify-signature",
            "--signature", "dGVzdA==",
            "--public-key-file", "nonexistent.pem",
        ])
        .assert()
        .failure();
}
