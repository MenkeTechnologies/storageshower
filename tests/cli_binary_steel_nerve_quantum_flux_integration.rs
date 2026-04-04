//! Binary smoke: `steel-nerve` and `quantum-flux` color flags.

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
fn steel_nerve_full_mount_version() {
    let o = output(&["--color", "steel-nerve", "-f", "-V"]);
    assert!(o.status.success());
}

#[test]
fn quantum_flux_no_header_help() {
    let o = output(&["--color", "quantum-flux", "--no-header", "--help"]);
    assert!(o.status.success());
}
