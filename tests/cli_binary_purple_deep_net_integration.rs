//! Binary smoke: `purple` and `deep-net` color flags.

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
fn purple_compact_list_colors() {
    let o = output(&["--color", "purple", "--compact", "--list-colors"]);
    assert!(o.status.success());
}

#[test]
fn deep_net_no_header_help() {
    let o = output(&["--color", "deep-net", "--no-header", "--help"]);
    assert!(o.status.success());
}
