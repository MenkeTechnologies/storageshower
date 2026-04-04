//! Binary smoke: `darkwave` and `matrix` color flags.

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
fn darkwave_bars_version() {
    let o = output(&["--color", "darkwave", "--bars", "-V"]);
    assert!(o.status.success());
}

#[test]
fn matrix_units_bytes_help() {
    let o = output(&["--color", "matrix", "--units", "bytes", "--help"]);
    assert!(o.status.success());
}
