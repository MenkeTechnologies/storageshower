//! Binary smoke: `toxic-waste` and `overlock` color flags.

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
fn toxic_waste_help() {
    let o = output(&["--color", "toxic-waste", "--help"]);
    assert!(o.status.success());
}

#[test]
fn overlock_sort_pct_version() {
    let o = output(&["--color", "overlock", "--sort", "pct", "-V"]);
    assert!(o.status.success());
}
