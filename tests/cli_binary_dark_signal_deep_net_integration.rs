//! Binary smoke: `dark-signal` and `deep-net` color flags.

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
fn dark_signal_compact_help() {
    let o = output(&["--color", "dark-signal", "-k", "--help"]);
    assert!(o.status.success());
}

#[test]
fn deep_net_version_reverse() {
    let o = output(&["--color", "deep-net", "-R", "-V"]);
    assert!(o.status.success());
}
