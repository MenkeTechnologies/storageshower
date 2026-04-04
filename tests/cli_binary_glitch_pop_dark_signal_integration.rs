//! Binary smoke: `glitch-pop` and `dark-signal` color flags.

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
fn glitch_pop_virtual_version() {
    let o = output(&["--color", "glitch-pop", "--virtual", "-V"]);
    assert!(o.status.success());
}

#[test]
fn dark_signal_compact_version() {
    let o = output(&["--color", "dark-signal", "--compact", "-V"]);
    assert!(o.status.success());
}
