//! Binary smoke: `sakura` and `neon-noir` color flags.

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
fn sakura_tooltips_version() {
    let o = output(&["--color", "sakura", "--tooltips", "-V"]);
    assert!(o.status.success());
}

#[test]
fn neon_noir_full_mount_version() {
    let o = output(&["--color", "neon-noir", "--full-mount", "-V"]);
    assert!(o.status.success());
}
