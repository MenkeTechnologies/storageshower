//! Binary smoke: `sunset` and `zaibatsu` color flags.

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
fn sunset_bars_version() {
    let o = output(&["--color", "sunset", "--bars", "-V"]);
    assert!(o.status.success());
}

#[test]
fn zaibatsu_sort_name_help() {
    let o = output(&["--color", "zaibatsu", "--sort", "name", "--help"]);
    assert!(o.status.success());
}
