//! Binary smoke: `blue` and `purple` color flags.

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
fn blue_units_mib_version() {
    let o = output(&["--color", "blue", "-u", "mib", "-V"]);
    assert!(o.status.success());
}

#[test]
fn purple_header_version() {
    let o = output(&["--color", "purple", "--header", "-V"]);
    assert!(o.status.success());
}
