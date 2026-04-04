//! Binary smoke: `sakura` and `laser-grid` color flags.

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
fn sakura_no_header_version() {
    let o = output(&["--color", "sakura", "--no-header", "-V"]);
    assert!(o.status.success());
}

#[test]
fn laser_grid_export_theme_help() {
    let o = output(&["--color", "laser-grid", "--export-theme", "--help"]);
    assert!(o.status.success());
}
