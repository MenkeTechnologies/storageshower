//! Binary smoke: `dark-signal` and `cyber-frost` color flags.

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
fn dark_signal_version() {
    let o = output(&["--color", "dark-signal", "-V"]);
    assert!(o.status.success());
}

#[test]
fn cyber_frost_list_colors() {
    let o = output(&["--color", "cyber-frost", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}
