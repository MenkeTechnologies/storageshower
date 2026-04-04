//! Binary smoke: `chrome-heart` and `zaibatsu` color flags.

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
fn chrome_heart_tooltips_version() {
    let o = output(&["--color", "chrome-heart", "--tooltips", "-V"]);
    assert!(o.status.success());
}

#[test]
fn zaibatsu_no_border_help() {
    let o = output(&["--color", "zaibatsu", "--no-border", "--help"]);
    assert!(o.status.success());
}
