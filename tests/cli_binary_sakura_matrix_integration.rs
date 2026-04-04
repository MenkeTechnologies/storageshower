//! Binary smoke: `sakura` and `matrix` color flags.

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
fn sakura_sort_name_version() {
    let o = output(&["--color", "sakura", "-s", "name", "-V"]);
    assert!(o.status.success());
}

#[test]
fn matrix_sort_pct_version() {
    let o = output(&["--color", "matrix", "-s", "pct", "-V"]);
    assert!(o.status.success());
}
