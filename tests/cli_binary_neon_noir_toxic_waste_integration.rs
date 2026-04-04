//! Binary smoke: `neon-noir` and `toxic-waste` color flags.

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
fn neon_noir_sort_size_version() {
    let o = output(&["--color", "neon-noir", "-s", "size", "-V"]);
    assert!(o.status.success());
}

#[test]
fn toxic_waste_no_border_version() {
    let o = output(&["--color", "toxic-waste", "--no-border", "-V"]);
    assert!(o.status.success());
}
