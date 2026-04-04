//! Binary smoke: `deep-net` and `overlock` color flags.

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
fn deep_net_no_header_version() {
    let o = output(&["--color", "deep-net", "--no-header", "-V"]);
    assert!(o.status.success());
}

#[test]
fn overlock_export_theme_help() {
    let o = output(&["--color", "overlock", "--export-theme", "--help"]);
    assert!(o.status.success());
}
