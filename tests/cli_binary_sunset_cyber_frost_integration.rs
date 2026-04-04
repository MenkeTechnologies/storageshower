//! Binary smoke: `sunset` and `cyber-frost` color flags.

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
fn sunset_full_mount_version() {
    let o = output(&["--color", "sunset", "-f", "-V"]);
    assert!(o.status.success());
}

#[test]
fn cyber_frost_bar_gradient_help() {
    let o = output(&["--color", "cyber-frost", "-b", "gradient", "--help"]);
    assert!(o.status.success());
}
