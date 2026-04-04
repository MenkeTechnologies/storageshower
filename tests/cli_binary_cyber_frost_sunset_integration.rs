//! Binary smoke: `cyber-frost` and `sunset` color flags.

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
fn cyber_frost_reverse_version() {
    let o = output(&["--color", "cyber-frost", "-R", "-V"]);
    assert!(o.status.success());
}

#[test]
fn sunset_theme_help() {
    let o = output(&["--color", "sunset", "--theme", "neon_theme", "--help"]);
    assert!(o.status.success());
}
