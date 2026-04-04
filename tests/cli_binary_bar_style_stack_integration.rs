//! Binary smoke: each `--bar-style` with safe exits (`CARGO_BIN_EXE_storageshower`).

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
fn gradient_version() {
    let o = output(&["--bar-style", "gradient", "-V"]);
    assert!(o.status.success());
}

#[test]
fn solid_help() {
    let o = output(&["-b", "solid", "--help"]);
    assert!(o.status.success());
    assert!(String::from_utf8_lossy(&o.stdout).len() > 80);
}

#[test]
fn thin_list_colors() {
    let o = output(&["--bar-style", "thin", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn ascii_sort_name_config_dev_null() {
    let o = output(&[
        "-b",
        "ascii",
        "--sort",
        "name",
        "--config",
        "/dev/null",
        "-V",
    ]);
    assert!(o.status.success());
}
