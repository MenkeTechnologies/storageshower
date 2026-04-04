//! Binary smoke: `steel-nerve` and `toxic-waste` color flags.

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
fn steel_nerve_no_header_version() {
    let o = output(&["--color", "steel-nerve", "--no-header", "-V"]);
    assert!(o.status.success());
}

#[test]
fn toxic_waste_virtual_version() {
    let o = output(&["--color", "toxic-waste", "--virtual", "-V"]);
    assert!(o.status.success());
}
