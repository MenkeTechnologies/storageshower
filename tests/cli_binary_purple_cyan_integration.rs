//! Binary smoke: `purple` and `cyan` color flags.

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
fn purple_local_only_version() {
    let o = output(&["--color", "purple", "-l", "-V"]);
    assert!(o.status.success());
}

#[test]
fn cyan_bar_solid_help() {
    let o = output(&["--color", "cyan", "-b", "solid", "--help"]);
    assert!(o.status.success());
}
