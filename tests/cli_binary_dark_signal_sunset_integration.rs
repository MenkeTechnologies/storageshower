//! Binary smoke: `dark-signal` and `sunset` color flags.

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
fn dark_signal_compact_version() {
    let o = output(&["--color", "dark-signal", "-k", "-V"]);
    assert!(o.status.success());
}

#[test]
fn sunset_bars_help() {
    let o = output(&["--color", "sunset", "--bars", "--help"]);
    assert!(o.status.success());
}
