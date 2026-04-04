//! Binary smoke: `chrome-heart` and `quantum-flux` color flags.

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
fn chrome_heart_no_virtual_version() {
    let o = output(&["--color", "chrome-heart", "--no-virtual", "-V"]);
    assert!(o.status.success());
}

#[test]
fn quantum_flux_warn_help() {
    let o = output(&["--color", "quantum-flux", "-w", "65", "--help"]);
    assert!(o.status.success());
}
