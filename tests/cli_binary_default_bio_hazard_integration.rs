//! Binary smoke: `default` and `bio-hazard` color flags.

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
fn default_no_compact_version() {
    let o = output(&["--color", "default", "--no-compact", "-V"]);
    assert!(o.status.success());
}

#[test]
fn bio_hazard_refresh_help() {
    let o = output(&["--color", "bio-hazard", "-r", "6", "--help"]);
    assert!(o.status.success());
}
