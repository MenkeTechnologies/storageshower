//! Binary smoke: `darkwave` and `toxic-waste` color flags.

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
fn darkwave_header_version() {
    let o = output(&["--color", "darkwave", "--header", "-V"]);
    assert!(o.status.success());
}

#[test]
fn toxic_waste_list_colors_help() {
    let o = output(&["--color", "toxic-waste", "--list-colors", "--help"]);
    assert!(o.status.success());
}
