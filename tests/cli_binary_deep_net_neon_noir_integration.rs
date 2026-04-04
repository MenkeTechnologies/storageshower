//! Binary smoke: `deep-net` and `neon-noir` color flags.

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
fn deep_net_reverse_version() {
    let o = output(&["--color", "deep-net", "-R", "-V"]);
    assert!(o.status.success());
}

#[test]
fn neon_noir_used_help() {
    let o = output(&["--color", "neon-noir", "--used", "--help"]);
    assert!(o.status.success());
}
