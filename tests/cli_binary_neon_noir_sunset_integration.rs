//! Binary smoke: `neon-noir` and `sunset` color flags.

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
fn neon_noir_tooltips_version() {
    let o = output(&["--color", "neon-noir", "--tooltips", "-V"]);
    assert!(o.status.success());
}

#[test]
fn sunset_no_border_help() {
    let o = output(&["--color", "sunset", "--no-border", "--help"]);
    assert!(o.status.success());
}
