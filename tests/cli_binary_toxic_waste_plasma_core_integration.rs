//! Binary smoke: `toxic-waste` and `plasma-core` color flags.

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
fn toxic_waste_no_header_version() {
    let o = output(&["--color", "toxic-waste", "--no-header", "-V"]);
    assert!(o.status.success());
}

#[test]
fn plasma_core_export_theme_help() {
    let o = output(&["--color", "plasma-core", "--export-theme", "--help"]);
    assert!(o.status.success());
}
