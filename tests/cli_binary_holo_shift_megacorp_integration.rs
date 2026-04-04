//! Binary smoke: `holo-shift` and `megacorp` color flags.

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
fn holo_shift_sort_name_version() {
    let o = output(&["--color", "holo-shift", "-s", "name", "-V"]);
    assert!(o.status.success());
}

#[test]
fn megacorp_no_tooltips_help() {
    let o = output(&["--color", "megacorp", "--no-tooltips", "--help"]);
    assert!(o.status.success());
}
