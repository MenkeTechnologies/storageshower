//! Binary smoke: `blade-runner` and `quantum-flux` color flags.

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
fn blade_runner_col_bar_end_version() {
    let o = output(&["--color", "blade-runner", "--col-bar-end", "18", "-V"]);
    assert!(o.status.success());
}

#[test]
fn quantum_flux_no_used_help() {
    let o = output(&["--color", "quantum-flux", "--no-used", "--help"]);
    assert!(o.status.success());
}
