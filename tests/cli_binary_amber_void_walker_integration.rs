//! Binary smoke: `amber` and `void-walker` color flags.

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
fn amber_bar_thin_version() {
    let o = output(&["--color", "amber", "-b", "thin", "-V"]);
    assert!(o.status.success());
}

#[test]
fn void_walker_config_help() {
    let o = output(&["--color", "void-walker", "-c", "/dev/null", "--help"]);
    assert!(o.status.success());
}
