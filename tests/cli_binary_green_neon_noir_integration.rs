//! Binary smoke: `green` and `neon-noir` color flags.

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
fn green_bars_version() {
    let o = output(&["--color", "green", "--bars", "-V"]);
    assert!(o.status.success());
}

#[test]
fn neon_noir_no_border_help() {
    let o = output(&["--color", "neon-noir", "--no-border", "--help"]);
    assert!(o.status.success());
}
