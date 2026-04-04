//! Binary smoke: `megacorp` and `void-walker` color flags.

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
fn megacorp_compact_version() {
    let o = output(&["--color", "megacorp", "--compact", "-V"]);
    assert!(o.status.success());
}

#[test]
fn void_walker_tooltips_version() {
    let o = output(&["--color", "void-walker", "--tooltips", "-V"]);
    assert!(o.status.success());
}
