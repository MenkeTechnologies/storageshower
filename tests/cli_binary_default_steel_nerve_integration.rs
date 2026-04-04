//! Binary smoke: `default` and `steel-nerve` color flags.

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
fn default_col_mount_version() {
    let o = output(&["--color", "default", "--col-mount", "24", "-V"]);
    assert!(o.status.success());
}

#[test]
fn steel_nerve_list_colors() {
    let o = output(&["--color", "steel-nerve", "--list-colors"]);
    assert!(o.status.success());
}
