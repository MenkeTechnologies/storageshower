//! Binary smoke: `--no-bars`, `--no-border`, `--no-header` with safe exits.

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
fn no_bars_no_border_version() {
    let o = output(&["--no-bars", "--no-border", "-V"]);
    assert!(o.status.success());
}

#[test]
fn no_header_help() {
    let o = output(&["--no-header", "--help"]);
    assert!(o.status.success());
}

#[test]
fn no_bars_no_header_list_colors() {
    let o = output(&["--no-bars", "--no-header", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}
