//! Binary smoke: `bio-hazard` and `overlock` color flags.

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
fn bio_hazard_refresh_version() {
    let o = output(&["--color", "bio-hazard", "-r", "3", "-V"]);
    assert!(o.status.success());
}

#[test]
fn overlock_no_bars_help() {
    let o = output(&["--color", "overlock", "--no-bars", "--help"]);
    assert!(o.status.success());
}
