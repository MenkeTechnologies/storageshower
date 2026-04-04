//! Binary smoke: `holo-shift` and `glitch-pop` color flags.

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
fn holo_shift_version() {
    let o = output(&["--color", "holo-shift", "-V"]);
    assert!(o.status.success());
}

#[test]
fn glitch_pop_no_used_help() {
    let o = output(&["--color", "glitch-pop", "--no-used", "--help"]);
    assert!(o.status.success());
}
