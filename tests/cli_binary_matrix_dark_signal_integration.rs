//! Binary smoke: `matrix` and `dark-signal` color flags.

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
fn matrix_no_used_version() {
    let o = output(&["--color", "matrix", "--no-used", "-V"]);
    assert!(o.status.success());
}

#[test]
fn dark_signal_list_colors() {
    let o = output(&["--color", "dark-signal", "--list-colors"]);
    assert!(o.status.success());
}
