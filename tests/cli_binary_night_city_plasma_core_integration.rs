//! Binary smoke: `night-city` and `plasma-core` color flags.

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
fn night_city_no_compact_version() {
    let o = output(&["--color", "night-city", "--no-compact", "-V"]);
    assert!(o.status.success());
}

#[test]
fn plasma_core_header_version() {
    let o = output(&["--color", "plasma-core", "--header", "-V"]);
    assert!(o.status.success());
}
