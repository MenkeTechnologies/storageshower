//! Binary smoke: `blue` and `red` color flags.

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
fn blue_compact_version() {
    let o = output(&["--color", "blue", "--compact", "-V"]);
    assert!(o.status.success());
}

#[test]
fn red_tooltips_version() {
    let o = output(&["--color", "red", "--tooltips", "-V"]);
    assert!(o.status.success());
}
