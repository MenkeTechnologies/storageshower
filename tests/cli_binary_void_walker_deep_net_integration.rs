//! Binary smoke: `void-walker` and `deep-net` color flags.

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
fn void_walker_compact_version() {
    let o = output(&["--color", "void-walker", "-k", "-V"]);
    assert!(o.status.success());
}

#[test]
fn deep_net_no_used_help() {
    let o = output(&["--color", "deep-net", "--no-used", "--help"]);
    assert!(o.status.success());
}
