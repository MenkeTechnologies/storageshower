//! Binary smoke: `bio-hazard` and `darkwave` color flags.

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
fn bio_hazard_virtual_version() {
    let o = output(&["--color", "bio-hazard", "--virtual", "-V"]);
    assert!(o.status.success());
}

#[test]
fn darkwave_units_gib_help() {
    let o = output(&["--color", "darkwave", "-u", "gib", "--help"]);
    assert!(o.status.success());
}
