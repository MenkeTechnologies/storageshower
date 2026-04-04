//! Binary smoke: `red` (Red Sector) and `bio-hazard` color flags.

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
fn red_refresh_version() {
    let o = output(&["--color", "red", "-r", "2", "-V"]);
    assert!(o.status.success());
}

#[test]
fn bio_hazard_list_colors() {
    let o = output(&["--color", "bio-hazard", "--list-colors"]);
    assert!(o.status.success());
}
