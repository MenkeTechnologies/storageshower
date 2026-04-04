//! Binary smoke: `green` and `dark-signal` color flags.

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
fn green_no_header_version() {
    let o = output(&["--color", "green", "--no-header", "-V"]);
    assert!(o.status.success());
}

#[test]
fn dark_signal_export_theme_help() {
    let o = output(&["--color", "dark-signal", "--export-theme", "--help"]);
    assert!(o.status.success());
}
