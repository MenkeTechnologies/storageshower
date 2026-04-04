//! Binary smoke: `quantum-flux` and `dark-signal` color flags.

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
fn quantum_flux_local_only_version() {
    let o = output(&["--color", "quantum-flux", "-l", "-V"]);
    assert!(o.status.success());
}

#[test]
fn dark_signal_no_used_help() {
    let o = output(&["--color", "dark-signal", "--no-used", "--help"]);
    assert!(o.status.success());
}
