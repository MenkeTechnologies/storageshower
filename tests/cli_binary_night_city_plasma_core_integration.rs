//! Binary smoke: `night-city` and `plasma-core` color flags.

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
fn plasma_core_sort_name_help() {
    let o = output(&["--color", "plasma-core", "-s", "name", "--help"]);
    assert!(o.status.success());
}
