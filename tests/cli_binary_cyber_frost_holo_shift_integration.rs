//! Binary smoke: `cyber-frost` and `holo-shift` color flags.

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
fn cyber_frost_no_border_version() {
    let o = output(&["--color", "cyber-frost", "--no-border", "-V"]);
    assert!(o.status.success());
}

#[test]
fn holo_shift_sort_size_version() {
    let o = output(&["--color", "holo-shift", "-s", "size", "-V"]);
    assert!(o.status.success());
}
