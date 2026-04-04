//! Binary smoke: `zaibatsu` and `chrome-heart` color flags.

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
fn zaibatsu_warn_crit_version() {
    let o = output(&["--color", "zaibatsu", "-w", "60", "-C", "92", "-V"]);
    assert!(o.status.success());
}

#[test]
fn chrome_heart_bar_style_gradient_help() {
    let o = output(&["--color", "chrome-heart", "-b", "gradient", "--help"]);
    assert!(o.status.success());
}
