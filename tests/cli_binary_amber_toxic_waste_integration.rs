//! Binary smoke: `amber` and `toxic-waste` color flags.

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
fn amber_warn_crit_version() {
    let o = output(&["--color", "amber", "-w", "50", "-C", "95", "-V"]);
    assert!(o.status.success());
}

#[test]
fn toxic_waste_no_virtual_help() {
    let o = output(&["--color", "toxic-waste", "--no-virtual", "--help"]);
    assert!(o.status.success());
}
