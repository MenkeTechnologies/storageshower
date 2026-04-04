//! Binary smoke: `glitch-pop` and `overlock` color flags.

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
fn overlock_no_full_mount_help() {
    let o = output(&["--color", "overlock", "--no-full-mount", "--help"]);
    assert!(o.status.success());
}
