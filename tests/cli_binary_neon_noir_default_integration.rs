//! Binary smoke: `neon-noir` and `default` color flags.

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
fn neon_noir_help() {
    let o = output(&["--color", "neon-noir", "--help"]);
    assert!(o.status.success());
}

#[test]
fn default_color_compact_list_colors() {
    let o = output(&["--color", "default", "-k", "--list-colors"]);
    assert!(o.status.success());
}
