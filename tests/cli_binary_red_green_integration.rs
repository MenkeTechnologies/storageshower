//! Binary smoke: `red` and `green` color flags.

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
fn red_sort_pct_version() {
    let o = output(&["--color", "red", "-s", "pct", "-V"]);
    assert!(o.status.success());
}

#[test]
fn green_header_version() {
    let o = output(&["--color", "green", "--header", "-V"]);
    assert!(o.status.success());
}
