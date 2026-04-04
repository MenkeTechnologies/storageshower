//! Binary smoke: `overlock` and `night-city` color flags.

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
fn overlock_local_only_version() {
    let o = output(&["--color", "overlock", "-l", "-V"]);
    assert!(o.status.success());
}

#[test]
fn night_city_col_mount_version() {
    let o = output(&["--color", "night-city", "--col-mount", "18", "-V"]);
    assert!(o.status.success());
}
