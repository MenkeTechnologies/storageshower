//! Binary smoke: `sakura` and `dark-signal` color flags.

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
fn sakura_no_bars_version() {
    let o = output(&["--color", "sakura", "--no-bars", "-V"]);
    assert!(o.status.success());
}

#[test]
fn dark_signal_units_human_help() {
    let o = output(&["--color", "dark-signal", "-u", "human", "--help"]);
    assert!(o.status.success());
}
