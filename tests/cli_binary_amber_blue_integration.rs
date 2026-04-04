//! Binary smoke: `amber` and `blue` color flags (Rust Belt / Ice Breaker palettes).

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
fn amber_bar_style_solid_version() {
    let o = output(&["--color", "amber", "-b", "solid", "-V"]);
    assert!(o.status.success());
}

#[test]
fn blue_refresh_config_dev_null() {
    let o = output(&[
        "--color",
        "blue",
        "-r",
        "4",
        "--config",
        "/dev/null",
        "--help",
    ]);
    assert!(o.status.success());
}
