//! Binary smoke: `default` and `green` color flags.

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
fn default_export_theme_version() {
    let o = output(&["--color", "default", "--export-theme", "-V"]);
    assert!(o.status.success());
}

#[test]
fn green_col_pct_version() {
    let o = output(&["--color", "green", "--col-pct", "16", "-V"]);
    assert!(o.status.success());
}
