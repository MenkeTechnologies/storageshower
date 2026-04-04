//! Binary smoke: `amber` and `laser-grid` color flags.

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
fn amber_warn_crit_version() {
    let o = output(&["--color", "amber", "-w", "60", "-C", "92", "-V"]);
    assert!(o.status.success());
}

#[test]
fn laser_grid_compact_help() {
    let o = output(&["--color", "laser-grid", "-k", "--help"]);
    assert!(o.status.success());
}
