//! Binary smoke: `zaibatsu` and `void-walker` color flags.

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
fn zaibatsu_no_border_version() {
    let o = output(&["--color", "zaibatsu", "--no-border", "-V"]);
    assert!(o.status.success());
}

#[test]
fn void_walker_sort_pct_help() {
    let o = output(&["--color", "void-walker", "--sort", "pct", "--help"]);
    assert!(o.status.success());
}
