use assert_cmd::Command;
use predicates::str::contains;
use lambert_w_function::lambert;

#[test]
fn test_valid_input() {
    let mut cmd = Command::cargo_bin("lambert_w_function").unwrap();
    cmd.arg("0.05")
        .assert()
        .success()
        .stdout(contains("Lambert W(0.05)"));
}

#[test]
fn test_out_of_domain_input() {
    let mut cmd = Command::cargo_bin("lambert_w_function").unwrap();
    cmd.arg("-1.0")
        .assert()
        .failure();
}

#[test]
fn test_lambert_function() {
    let result = lambert::lambert_w(0.05);
    assert!(result.is_ok(), "Function should return a valid result");
}