//! Binary smoke: `cyber-frost` and `matrix` color flags.

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
fn cyber_frost_no_tooltips_version() {
    let o = output(&["--color", "cyber-frost", "--no-tooltips", "-V"]);
    assert!(o.status.success());
}

#[test]
fn matrix_bar_solid_help() {
    let o = output(&["--color", "matrix", "-b", "solid", "--help"]);
    assert!(o.status.success());
}
