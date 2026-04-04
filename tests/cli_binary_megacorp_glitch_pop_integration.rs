//! Binary smoke: `megacorp` and `glitch-pop` color flags.

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
fn megacorp_units_gib_version() {
    let o = output(&["--color", "megacorp", "-u", "gib", "-V"]);
    assert!(o.status.success());
}

#[test]
fn glitch_pop_no_header_help() {
    let o = output(&["--color", "glitch-pop", "--no-header", "--help"]);
    assert!(o.status.success());
}
