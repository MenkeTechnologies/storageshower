//! Binary smoke: `quantum-flux` and `bio-hazard` color flags.

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
fn quantum_flux_list_colors_version() {
    let o = output(&["--color", "quantum-flux", "--list-colors", "-V"]);
    assert!(o.status.success());
}

#[test]
fn bio_hazard_sort_size_help() {
    let o = output(&["--color", "bio-hazard", "--sort", "size", "--help"]);
    assert!(o.status.success());
}
