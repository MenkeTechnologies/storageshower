//! Binary smoke: `void-walker` and `neon-noir` color flags.

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
fn void_walker_list_colors() {
    let o = output(&["--color", "void-walker", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn neon_noir_version() {
    let o = output(&["--color", "neon-noir", "-V"]);
    assert!(o.status.success());
}

#[test]
fn void_walker_units_gib_help() {
    let o = output(&["--color", "void-walker", "--units", "gib", "--help"]);
    assert!(o.status.success());
}
