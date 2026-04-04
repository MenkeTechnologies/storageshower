//! Binary smoke: `green` and `cyan` color flags.

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
fn green_no_used_version() {
    let o = output(&["--color", "green", "--no-used", "-V"]);
    assert!(o.status.success());
}

#[test]
fn cyan_col_bar_end_version() {
    let o = output(&["--color", "cyan", "--col-bar-end", "22", "-V"]);
    assert!(o.status.success());
}
