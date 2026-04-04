//! Binary smoke: `red` (Red Sector) and `plasma-core` color flags.

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
fn red_border_version() {
    let o = output(&["--color", "red", "--border", "-V"]);
    assert!(o.status.success());
}

#[test]
fn plasma_core_col_pct_help() {
    let o = output(&["--color", "plasma-core", "--col-pct", "15", "--help"]);
    assert!(o.status.success());
}
