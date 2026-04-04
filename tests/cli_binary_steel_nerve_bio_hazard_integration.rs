//! Binary smoke: `steel-nerve` and `bio-hazard` color flags.

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
fn steel_nerve_no_used_version() {
    let o = output(&["--color", "steel-nerve", "--no-used", "-V"]);
    assert!(o.status.success());
}

#[test]
fn bio_hazard_col_bar_end_help() {
    let o = output(&["--color", "bio-hazard", "--col-bar-end", "12", "--help"]);
    assert!(o.status.success());
}
