//! Binary smoke: `zaibatsu` and `neon-noir` color flags.

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
fn zaibatsu_no_header_version() {
    let o = output(&["--color", "zaibatsu", "--no-header", "-V"]);
    assert!(o.status.success());
}

#[test]
fn neon_noir_units_mib_help() {
    let o = output(&["--color", "neon-noir", "-u", "mib", "--help"]);
    assert!(o.status.success());
}
