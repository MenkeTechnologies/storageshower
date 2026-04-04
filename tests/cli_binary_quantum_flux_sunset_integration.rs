//! Binary smoke: `quantum-flux` and `sunset` color flags.

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
fn quantum_flux_list_colors() {
    let o = output(&["--color", "quantum-flux", "--list-colors"]);
    assert!(o.status.success());
}

#[test]
fn sunset_no_virtual_version() {
    let o = output(&["--color", "sunset", "--no-virtual", "-V"]);
    assert!(o.status.success());
}
