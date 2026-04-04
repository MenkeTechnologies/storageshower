//! Binary smoke: `matrix` and `deep-net` color flags.

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
fn matrix_header_version() {
    let o = output(&["--color", "matrix", "--header", "-V"]);
    assert!(o.status.success());
}

#[test]
fn deep_net_sort_pct_version() {
    let o = output(&["--color", "deep-net", "-s", "pct", "-V"]);
    assert!(o.status.success());
}
