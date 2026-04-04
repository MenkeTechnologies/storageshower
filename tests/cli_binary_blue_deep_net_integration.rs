//! Binary smoke: `blue` and `deep-net` color flags.

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
fn blue_no_virtual_version() {
    let o = output(&["--color", "blue", "--no-virtual", "-V"]);
    assert!(o.status.success());
}

#[test]
fn deep_net_sort_name_help() {
    let o = output(&["--color", "deep-net", "-s", "name", "--help"]);
    assert!(o.status.success());
}
