//! Binary smoke: `zaibatsu` and `sunset` color flags.

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
fn zaibatsu_no_used_version() {
    let o = output(&["--color", "zaibatsu", "--no-used", "-V"]);
    assert!(o.status.success());
}

#[test]
fn sunset_list_colors_version() {
    let o = output(&["--color", "sunset", "--list-colors", "-V"]);
    assert!(o.status.success());
}
