//! Binary smoke: `holo-shift` and `cyber-frost` color flags.

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
fn holo_shift_local_only_version() {
    let o = output(&["--color", "holo-shift", "-l", "-V"]);
    assert!(o.status.success());
}

#[test]
fn cyber_frost_no_tooltips_help() {
    let o = output(&["--color", "cyber-frost", "--no-tooltips", "--help"]);
    assert!(o.status.success());
}
