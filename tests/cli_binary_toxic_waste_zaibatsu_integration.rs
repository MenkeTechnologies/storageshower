//! Binary smoke: `toxic-waste` and `zaibatsu` color flags.

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
fn toxic_waste_version() {
    let o = output(&["--color", "toxic-waste", "-V"]);
    assert!(o.status.success());
}

#[test]
fn zaibatsu_help_full_mount() {
    let o = output(&["--color", "zaibatsu", "-f", "--help"]);
    assert!(o.status.success());
}
