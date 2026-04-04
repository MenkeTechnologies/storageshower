//! Binary smoke: `quantum-flux` and `glitch-pop` color flags.

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
fn quantum_flux_version() {
    let o = output(&["--color", "quantum-flux", "-V"]);
    assert!(o.status.success());
}

#[test]
fn glitch_pop_help() {
    let o = output(&["--color", "glitch-pop", "--help"]);
    assert!(o.status.success());
}
