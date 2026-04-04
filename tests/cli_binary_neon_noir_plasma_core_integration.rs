//! Binary smoke: `neon-noir` and `plasma-core` color flags.

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
fn neon_noir_export_theme_version() {
    let o = output(&["--color", "neon-noir", "--export-theme", "-V"]);
    assert!(o.status.success());
}

#[test]
fn plasma_core_full_mount_help() {
    let o = output(&["--color", "plasma-core", "-f", "--help"]);
    assert!(o.status.success());
}
