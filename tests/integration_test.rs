use assert_cmd::{assert::Assert, prelude::*};
use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};
use std::{fs, path::Path, process::Command};

const EXIT_CODE_ERROR: i32 = 1;

#[fixture]
fn cmd() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

fn cleanup() {
    if Path::new("fortune_slips.pdf").exists() {
        fs::remove_file("fortune_slips.pdf").unwrap();
    }
}

#[rstest]
fn test_cli_with_correct_config(mut cmd: Command) {
    cmd.arg("--config")
        .arg("test_utils/data/fortune_settings.yaml");

    assert_cmd_and_ok_x(&mut cmd, 3, 0);
    assert!(Path::new("fortune_slips.pdf").exists());
    cleanup();
}

#[rstest]
fn test_cli_with_correct_config_and_custom_output(mut cmd: Command) {
    cmd.arg("--config")
        .arg("test_utils/data/fortune_settings.yaml")
        .arg("--output")
        .arg("custom_output.pdf");

    assert_cmd_and_ok_x(&mut cmd, 3, 0);

    assert!(Path::new("custom_output.pdf").exists());
    fs::remove_file("custom_output.pdf").unwrap();
}

#[rstest]
fn test_cli_default_should_fail(mut cmd: Command) {
    assert_cmd_and_ok_x(&mut cmd, 2, 1);
}

#[rstest]
fn test_cli_with_non_existing_config_should_fail(mut cmd: Command) {
    cmd.arg("--config")
        .arg("test_utils/data/does_not_exist.yaml");

    assert_cmd_and_ok_x(&mut cmd, 2, 1);
}

#[rstest]
fn test_cli_with_invalid_config_should_fail(mut cmd: Command) {
    cmd.arg("--config")
        .arg("test_utils/data/invalid_fortune_settings.yaml");

    assert_cmd_and_ok_x(&mut cmd, 2, 1);
}

#[rstest]
fn test_cli_skip_checks(mut cmd: Command) {
    cmd.arg("--skip-checks")
        .arg("--config")
        .arg("test_utils/data/fortune_settings.yaml");

    let assert = cmd.assert().success();

    let checks_count = count_ok_and_error_in_prereq_checks(&assert);
    assert_eq!(checks_count.ok, 0);
    assert_eq!(checks_count.error, 0);
    assert!(Path::new("fortune_slips.pdf").exists());

    cleanup();
}

#[rstest]
fn test_cli_verbose(mut cmd: Command) {
    cmd.arg("--verbose")
        .arg("--config")
        .arg("test_utils/data/fortune_settings.yaml");

    assert_cmd_and_ok_x(&mut cmd, 3, 0);

    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("DEBUG"));

    cleanup();
}

fn assert_cmd_and_ok_x(cmd: &mut Command, ok: usize, x: usize) {
    let assert = match ok {
        0..=2 => cmd.assert().failure().code(EXIT_CODE_ERROR),
        3 => cmd.assert().success(),
        _ => panic!("Invalid number of OK checks"),
    };

    let checks_count = count_ok_and_error_in_prereq_checks(&assert);
    assert_eq!(checks_count.ok, ok);
    assert_eq!(checks_count.error, x);
}

struct ChecksCount {
    ok: usize,
    error: usize,
}

fn count_ok_and_error_in_prereq_checks(assert: &Assert) -> ChecksCount {
    let output = String::from_utf8_lossy(&assert.get_output().stdout);
    let nr_ok = output.matches(&" OK\n").count();
    let nr_x = output.matches(&" Error ").count();
    ChecksCount {
        ok: nr_ok,
        error: nr_x,
    }
}
