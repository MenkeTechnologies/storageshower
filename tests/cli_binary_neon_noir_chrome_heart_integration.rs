//! Binary smoke: `neon-noir` and `chrome-heart` color flags.

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
fn neon_noir_no_bars_version() {
    let o = output(&["--color", "neon-noir", "--no-bars", "-V"]);
    assert!(o.status.success());
}

#[test]
fn chrome_heart_config_dev_null_version() {
    let o = output(&["--color", "chrome-heart", "--config", "/dev/null", "-V"]);
    assert!(o.status.success());
}
