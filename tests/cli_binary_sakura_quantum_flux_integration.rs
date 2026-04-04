//! Binary smoke: `sakura` and `quantum-flux` color flags.

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
fn sakura_no_bars_version() {
    let o = output(&["--color", "sakura", "--no-bars", "-V"]);
    assert!(o.status.success());
}

#[test]
fn quantum_flux_col_pct_help() {
    let o = output(&["--color", "quantum-flux", "--col-pct", "12", "--help"]);
    assert!(o.status.success());
}
