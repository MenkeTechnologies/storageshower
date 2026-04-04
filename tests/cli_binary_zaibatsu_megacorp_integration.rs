//! Binary smoke: `zaibatsu` and `megacorp` color flags.

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
fn zaibatsu_theme_export_version() {
    let o = output(&["--color", "zaibatsu", "--export-theme", "-V"]);
    assert!(o.status.success());
}

#[test]
fn megacorp_no_compact_version() {
    let o = output(&["--color", "megacorp", "--no-compact", "-V"]);
    assert!(o.status.success());
}
