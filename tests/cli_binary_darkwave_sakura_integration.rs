//! Binary smoke: `darkwave` and `sakura` color flags.

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
fn darkwave_config_dev_null_help() {
    let o = output(&["--color", "darkwave", "--config", "/dev/null", "--help"]);
    assert!(o.status.success());
}

#[test]
fn sakura_export_theme_parse() {
    let o = output(&["--color", "sakura", "--export-theme", "-V"]);
    assert!(o.status.success());
}
