//! Binary smoke: `laser-grid` and `quantum-flux` color flags.

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
fn laser_grid_version() {
    let o = output(&["--color", "laser-grid", "-V"]);
    assert!(o.status.success());
}

#[test]
fn quantum_flux_help() {
    let o = output(&["--color", "quantum-flux", "--help"]);
    assert!(o.status.success());
}
