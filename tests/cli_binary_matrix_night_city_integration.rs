//! Binary smoke: `matrix` and `night-city` color flags.

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
fn matrix_sort_pct_version() {
    let o = output(&["--color", "matrix", "-s", "pct", "-V"]);
    assert!(o.status.success());
}

#[test]
fn night_city_no_reverse_help() {
    let o = output(&["--color", "night-city", "--no-reverse", "--help"]);
    assert!(o.status.success());
}
