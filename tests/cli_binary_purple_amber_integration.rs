//! Binary smoke: `purple` and `amber` color flags.

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
fn purple_units_bytes_version() {
    let o = output(&["--color", "purple", "-u", "bytes", "-V"]);
    assert!(o.status.success());
}

#[test]
fn amber_no_virtual_version() {
    let o = output(&["--color", "amber", "--no-virtual", "-V"]);
    assert!(o.status.success());
}
