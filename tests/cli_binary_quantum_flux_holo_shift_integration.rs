//! Binary smoke: `quantum-flux` and `holo-shift` color flags.

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
fn quantum_flux_export_theme_version() {
    let o = output(&["--color", "quantum-flux", "--export-theme", "-V"]);
    assert!(o.status.success());
}

#[test]
fn holo_shift_refresh_help() {
    let o = output(&["--color", "holo-shift", "-r", "4", "--help"]);
    assert!(o.status.success());
}
