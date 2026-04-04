//! Binary smoke: `quantum-flux` and `night-city` color flags.

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
fn quantum_flux_no_header_version() {
    let o = output(&["--color", "quantum-flux", "--no-header", "-V"]);
    assert!(o.status.success());
}

#[test]
fn night_city_export_theme_help() {
    let o = output(&["--color", "night-city", "--export-theme", "--help"]);
    assert!(o.status.success());
}
