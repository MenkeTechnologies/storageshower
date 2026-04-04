//! Binary smoke: `--local-only`, `--virtual`, `--refresh` with safe exits.

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
fn local_only_version() {
    let o = output(&["--local-only", "-V"]);
    assert!(o.status.success());
}

#[test]
fn virtual_refresh_help() {
    let o = output(&["--virtual", "--refresh", "2", "--help"]);
    assert!(o.status.success());
}

#[test]
fn local_only_units_gib_list_colors() {
    let o = output(&["-l", "--units", "gib", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn no_virtual_compact_version() {
    let o = output(&["--no-virtual", "-k", "-V"]);
    assert!(o.status.success());
}
