//! Binary smoke: `sakura` and `purple` color flags.

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
fn sakura_no_virtual_version() {
    let o = output(&["--color", "sakura", "--no-virtual", "-V"]);
    assert!(o.status.success());
}

#[test]
fn purple_col_mount_version() {
    let o = output(&["--color", "purple", "--col-mount", "24", "-V"]);
    assert!(o.status.success());
}
