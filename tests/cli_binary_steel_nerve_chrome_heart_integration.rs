//! Binary smoke: `steel-nerve` and `chrome-heart` color flags.

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
fn steel_nerve_version() {
    let o = output(&["--color", "steel-nerve", "-V"]);
    assert!(o.status.success());
}

#[test]
fn chrome_heart_help() {
    let o = output(&["--color", "chrome-heart", "--help"]);
    assert!(o.status.success());
}
