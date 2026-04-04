//! Binary smoke: `deep-net` and `bio-hazard` color flags.

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
fn deep_net_sort_pct_version() {
    let o = output(&["--color", "deep-net", "--sort", "pct", "-V"]);
    assert!(o.status.success());
}

#[test]
fn bio_hazard_no_border_help() {
    let o = output(&["--color", "bio-hazard", "--no-border", "--help"]);
    assert!(o.status.success());
}
