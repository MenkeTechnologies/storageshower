//! Binary smoke: `holo-shift` and `quantum-flux` color flags.

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
fn holo_shift_compact_version() {
    let o = output(&["--color", "holo-shift", "-k", "-V"]);
    assert!(o.status.success());
}

#[test]
fn quantum_flux_sort_pct_help() {
    let o = output(&["--color", "quantum-flux", "-s", "pct", "--help"]);
    assert!(o.status.success());
}
