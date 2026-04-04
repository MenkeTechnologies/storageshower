//! Binary smoke: `night-city` and `deep-net` color flags.

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
fn night_city_sort_pct_version() {
    let o = output(&["--color", "night-city", "-s", "pct", "-V"]);
    assert!(o.status.success());
}

#[test]
fn deep_net_compact_version() {
    let o = output(&["--color", "deep-net", "--compact", "-V"]);
    assert!(o.status.success());
}
