//! Binary smoke: `holo-shift` and `toxic-waste` color flags.

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
fn holo_shift_virtual_version() {
    let o = output(&["--color", "holo-shift", "--virtual", "-V"]);
    assert!(o.status.success());
}

#[test]
fn toxic_waste_col_pct_help() {
    let o = output(&["--color", "toxic-waste", "--col-pct", "7", "--help"]);
    assert!(o.status.success());
}
