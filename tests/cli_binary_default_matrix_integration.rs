//! Binary smoke: `default` and `matrix` color flags.

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
fn default_list_colors_version() {
    let o = output(&["--color", "default", "--list-colors", "-V"]);
    assert!(o.status.success());
}

#[test]
fn matrix_border_version() {
    let o = output(&["--color", "matrix", "--border", "-V"]);
    assert!(o.status.success());
}
