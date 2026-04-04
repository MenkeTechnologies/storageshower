//! Binary smoke: `overlock` and `darkwave` color flags.

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
fn overlock_reverse_version() {
    let o = output(&["--color", "overlock", "-R", "-V"]);
    assert!(o.status.success());
}

#[test]
fn darkwave_units_mib_help() {
    let o = output(&["--color", "darkwave", "--units", "mib", "--help"]);
    assert!(o.status.success());
}
