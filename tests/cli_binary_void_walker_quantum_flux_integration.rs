//! Binary smoke: `void-walker` and `quantum-flux` color flags.

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
fn void_walker_full_mount_version() {
    let o = output(&["--color", "void-walker", "--full-mount", "-V"]);
    assert!(o.status.success());
}

#[test]
fn quantum_flux_no_border_version() {
    let o = output(&["--color", "quantum-flux", "--no-border", "-V"]);
    assert!(o.status.success());
}
