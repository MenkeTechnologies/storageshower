//! Binary smoke: `matrix` and `bio-hazard` color flags.

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
fn matrix_tooltips_version() {
    let o = output(&["--color", "matrix", "--tooltips", "-V"]);
    assert!(o.status.success());
}

#[test]
fn bio_hazard_no_bars_help() {
    let o = output(&["--color", "bio-hazard", "--no-bars", "--help"]);
    assert!(o.status.success());
}
