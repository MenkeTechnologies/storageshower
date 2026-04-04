//! Binary smoke: `default` and `purple` color flags.

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
fn default_no_bars_version() {
    let o = output(&["--color", "default", "--no-bars", "-V"]);
    assert!(o.status.success());
}

#[test]
fn purple_sort_name_version() {
    let o = output(&["--color", "purple", "-s", "name", "-V"]);
    assert!(o.status.success());
}
