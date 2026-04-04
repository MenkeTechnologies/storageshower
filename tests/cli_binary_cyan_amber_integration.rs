//! Binary smoke: `cyan` and `amber` color flags.

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
fn cyan_refresh_help() {
    let o = output(&["--color", "cyan", "-r", "2", "--help"]);
    assert!(o.status.success());
}

#[test]
fn amber_sort_pct_version() {
    let o = output(&["--color", "amber", "--sort", "pct", "-V"]);
    assert!(o.status.success());
}
