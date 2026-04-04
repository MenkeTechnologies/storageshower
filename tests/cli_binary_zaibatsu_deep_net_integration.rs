//! Binary smoke: `zaibatsu` and `deep-net` color flags.

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
fn zaibatsu_no_virtual_version() {
    let o = output(&["--color", "zaibatsu", "--no-virtual", "-V"]);
    assert!(o.status.success());
}

#[test]
fn deep_net_col_mount_help() {
    let o = output(&["--color", "deep-net", "--col-mount", "30", "--help"]);
    assert!(o.status.success());
}
