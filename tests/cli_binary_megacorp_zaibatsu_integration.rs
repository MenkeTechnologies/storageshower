//! Binary smoke: `megacorp` and `zaibatsu` color flags.

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
fn megacorp_version() {
    let o = output(&["--color", "megacorp", "-V"]);
    assert!(o.status.success());
}

#[test]
fn zaibatsu_list_colors() {
    let o = output(&["--color", "zaibatsu", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}
