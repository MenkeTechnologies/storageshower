//! Binary smoke: `toxic-waste` and `quantum-flux` color flags.

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
fn toxic_waste_sort_pct_version() {
    let o = output(&["--color", "toxic-waste", "-s", "pct", "-V"]);
    assert!(o.status.success());
}

#[test]
fn quantum_flux_no_used_version() {
    let o = output(&["--color", "quantum-flux", "--no-used", "-V"]);
    assert!(o.status.success());
}
