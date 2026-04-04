//! Binary smoke: `night-city` and `zaibatsu` color flags.

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
fn night_city_no_bars_version() {
    let o = output(&["--color", "night-city", "--no-bars", "-V"]);
    assert!(o.status.success());
}

#[test]
fn zaibatsu_sort_pct_help() {
    let o = output(&["--color", "zaibatsu", "--sort", "pct", "--help"]);
    assert!(o.status.success());
}
