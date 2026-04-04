//! Binary smoke: `cyan` and `matrix` color flags.

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
fn cyan_refresh_version() {
    let o = output(&["--color", "cyan", "-r", "5", "-V"]);
    assert!(o.status.success());
}

#[test]
fn matrix_no_header_version() {
    let o = output(&["--color", "matrix", "--no-header", "-V"]);
    assert!(o.status.success());
}
