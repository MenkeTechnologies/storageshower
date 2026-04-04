//! Binary smoke: `glitch-pop` and `neon-noir` color flags.

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
fn glitch_pop_border_version() {
    let o = output(&["--color", "glitch-pop", "--border", "-V"]);
    assert!(o.status.success());
}

#[test]
fn neon_noir_col_mount_help() {
    let o = output(&["--color", "neon-noir", "--col-mount", "18", "--help"]);
    assert!(o.status.success());
}
