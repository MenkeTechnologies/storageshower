//! Binary smoke: `night-city` and `blade-runner` color flags.

use std::process::Command;

fn exe() -> &'static str {
    env!("CARGO_BIN_EXE_storageshower")
}

fn output(args: &[&str]) -> std::process::Output {
    Command::new(exe())
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("spawn {}: {e}", exe()))
}

#[test]
fn night_city_no_header_version() {
    let o = output(&["--color", "night-city", "--no-header", "-V"]);
    assert!(o.status.success());
}

#[test]
fn blade_runner_sort_name_help() {
    let o = output(&["--color", "blade-runner", "--sort", "name", "--help"]);
    assert!(o.status.success());
}
